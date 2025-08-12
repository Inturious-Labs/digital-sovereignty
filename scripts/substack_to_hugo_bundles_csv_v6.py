#!/usr/bin/env python3
"""
Substack → Hugo Page Bundles (CSV-aware v6, downloads external images)
---------------------------------------------------------------------
- Folder layout: content/<section>/<YYYY>/<MM>/<DD>-<slug>/index.md
- Front matter: title, description, slug, date, draft, tags, categories, featured_image
- Copies local images referenced by the HTML into the bundle and rewrites paths.
- NEW: Downloads **external** images (http/https) into the same bundle and rewrites
       Markdown to point to the local file names.
- If featured_image is an external URL, it is downloaded and front matter points to ./filename.

Dependencies:
  pip install beautifulsoup4 markdownify python-dateutil requests

Usage:
  python substack_to_hugo_bundles_csv_v6.py --export . --csv ./posts.csv --out content --section posts --dry-run
  python substack_to_hugo_bundles_csv_v6.py --export . --csv ./posts.csv --out content --section posts
"""

import argparse
import zipfile
import tempfile
import shutil
import re
from pathlib import Path
from datetime import datetime
from typing import List, Tuple, Optional, Dict
from urllib.parse import urlparse
import mimetypes
import os

import csv
import requests
from requests.adapters import HTTPAdapter, Retry
from bs4 import BeautifulSoup
from markdownify import markdownify as md
from dateutil import parser as dateparser

# -----------------------------
# Helpers
# -----------------------------

def slugify(text: str) -> str:
    text = text.strip().lower()
    text = re.sub(r"[^\w\s-]", "", text, flags=re.UNICODE)
    text = re.sub(r"[\s_-]+", "-", text)
    text = re.sub(r"^-+|-+$", "", text)
    return text or "post"

def split_filename_parts(html_path: Path) -> tuple[Optional[int], Optional[str]]:
    stem = html_path.stem
    m = re.match(r"^(\d+)(?:\.(.*))?$", stem)
    if m:
        pid = int(m.group(1))
        s = (m.group(2) or "").strip().strip(".")
        return pid, (slugify(s) if s else None)
    return None, slugify(stem)

def find_posts_dir(root: Path) -> Path:
    candidates = [root / "posts", root / "post", root]
    for c in candidates:
        if c.exists() and any(p.suffix.lower() == ".html" for p in c.rglob("*.html")):
            return c
    return root

def extract_title_from_html(soup: BeautifulSoup) -> Optional[str]:
    og = soup.find("meta", attrs={"property": "og:title"})
    if og and og.get("content"):
        return og["content"].strip()
    if soup.title and soup.title.string:
        return soup.title.string.strip()
    h = soup.find(["h1", "h2"])
    if h and h.get_text(strip=True):
        return h.get_text(strip=True)
    return None

def extract_og_image(soup: BeautifulSoup) -> Optional[str]:
    og = soup.find("meta", attrs={"property": "og:image"})
    if og and og.get("content"):
        return og["content"].strip()
    return None

def extract_date_from_html(soup: BeautifulSoup, fallback: Optional[datetime]=None) -> datetime:
    t = soup.find("time", attrs={"datetime": True})
    if t and t.get("datetime"):
        try:
            return dateparser.parse(t["datetime"])
        except Exception:
            pass
    for key in ("article:published_time", "pubdate", "date", "publish_date"):
        m = soup.find("meta", attrs={"property": key}) or soup.find("meta", attrs={"name": key})
        if m and m.get("content"):
            try:
                return dateparser.parse(m["content"])
            except Exception:
                continue
    any_time = soup.find(attrs={"data-published-at": True}) or soup.find(attrs={"data-date": True})
    if any_time:
        val = any_time.get("data-published-at") or any_time.get("data-date")
        try:
            return dateparser.parse(val)
        except Exception:
            pass
    return fallback or datetime.now()

def extract_tags(soup: BeautifulSoup) -> List[str]:
    tags = []
    meta_kw = soup.find("meta", attrs={"name": "keywords"})
    if meta_kw and meta_kw.get("content"):
        tags.extend([t.strip() for t in meta_kw["content"].split(",") if t.strip()])
    for taglist in soup.find_all(attrs={"class": re.compile("tag", re.I)}):
        for a in taglist.find_all("a"):
            label = a.get_text(strip=True)
            if label and label.lower() not in [t.lower() for t in tags]:
                tags.append(label)
    uniq = []
    seen = set()
    for t in tags:
        key = t.lower()
        if key not in seen:
            seen.add(key)
            uniq.append(t)
    return uniq[:30]

def find_article_root(soup: BeautifulSoup):
    for sel in [
        "article",
        "div.post", "div.post-body", "div.postBody", "div.body",
        "div#content", "div.main", "section.post"
    ]:
        node = soup.select_one(sel)
        if node:
            return node
    return soup.body or soup

def collect_images(article_root, html_path: Path) -> List[Tuple[str, Optional[Path]]]:
    """
    Return list of (src, local_abs_path_if_local_else_None) for images.
    Local = exists relative to html file or in export's top-level images dir.
    External (http/https/data:) will have None path and non-empty src.
    """
    results = []
    for img in article_root.find_all("img"):
        src = (img.get("src") or "").strip()
        if not src:
            continue
        if src.startswith(("http://", "https://", "data:")):
            results.append((src, None))
            continue
        abs_path = (html_path.parent / src).resolve()
        if abs_path.exists():
            results.append((src, abs_path))
        else:
            alt = (html_path.parents[1] / "images" / Path(src).name) if len(html_path.parents) > 1 else None
            if alt and alt.exists():
                results.append((src, alt))
            else:
                # keep as unresolved local; skip copy but keep original src for replacement attempts
                results.append((src, None))
    return results

def html_to_markdown(article_html: str) -> str:
    return md(
        article_html,
        heading_style="ATX",
        bullets="*",
        strip=["script", "style"],
        code_language=None
    ).strip() + "\n"

def format_iso(dt: datetime) -> str:
    try:
        return dt.isoformat()
    except Exception:
        return datetime.now().isoformat()

def write_front_matter(title: str, date: datetime, slug: str,
                       draft: Optional[bool],
                       description: Optional[str],
                       tags: List[str],
                       categories: List[str],
                       featured_image: Optional[str]) -> str:
    lines = ["---"]
    safe_title = title.replace('"', '""')
    lines.append(f'title: "{safe_title}"')
    lines.append(f"date: {format_iso(date)}")
    lines.append(f"slug: {slug}")
    if draft is not None:
        lines.append(f"draft: {'true' if draft else 'false'}")
    if description:
        safe_desc = description.replace('"', '""')
        lines.append(f'description: "{safe_desc}"')
    if featured_image:
        safe_img = featured_image.replace('"', '""')
        lines.append(f'featured_image: "{safe_img}"')
    if categories:
        lines.append("categories:")
        for c in categories:
            safe = c.replace('"', '\\"')
            lines.append(f'  - "{safe}"')
    if tags:
        lines.append("tags:")
        for t in tags:
            safe = t.replace('"', '\\"')
            lines.append(f'  - "{safe}"')
    lines.append("---\n")
    return "\n".join(lines)

def build_http_session() -> requests.Session:
    session = requests.Session()
    retries = Retry(
        total=3,
        backoff_factor=0.3,
        status_forcelist=[429, 500, 502, 503, 504],
        allowed_methods=["GET", "HEAD"],
        raise_on_status=False
    )
    adapter = HTTPAdapter(max_retries=retries, pool_connections=8, pool_maxsize=8)
    session.mount("http://", adapter)
    session.mount("https://", adapter)
    session.headers.update({
        "User-Agent": "substack-to-hugo/1.0 (+https://example.com)"
    })
    return session

def pick_filename_from_url(url: str, content_type: Optional[str], default_stem: str, used_names: set[str]) -> str:
    """
    Choose a safe filename for a downloaded image.
    - Prefer the URL path's basename (without query).
    - If no extension, infer from content_type (e.g., image/jpeg -> .jpg).
    - Ensure uniqueness within used_names.
    """
    parsed = urlparse(url)
    name = os.path.basename(parsed.path)
    name = name.split("?")[0].split("#")[0]

    # Fallback name if empty
    if not name or "." not in name:
        ext = ""
        if content_type:
            ext = mimetypes.guess_extension(content_type.split(";")[0].strip()) or ""
        name = f"{default_stem}{ext or '.img'}"

    # Very long names → truncate
    if len(name) > 128:
        root, ext = os.path.splitext(name)
        name = root[:100] + ext

    base, ext = os.path.splitext(name)
    candidate = name
    i = 2
    while candidate in used_names:
        candidate = f"{base}-{i}{ext}"
        i += 1
    used_names.add(candidate)
    return candidate

# -----------------------------
# CSV mapping
# -----------------------------

def autodetect_csv(root: Path) -> Optional[Path]:
    for p in [root / "posts.csv", root / "post.csv", root / "newsletter.csv"]:
        if p.exists():
            return p
    for p in root.glob("*.csv"):
        try:
            with p.open("r", encoding="utf-8", errors="ignore") as f:
                head = f.readline().lower()
                if "post_id" in head:
                    return p
        except Exception:
            continue
    return None

def build_maps_from_csv(csv_path: Path):
    dates_by_id: Dict[int, datetime] = {}
    dates_by_slug: Dict[str, datetime] = {}

    draft_by_id: Dict[int, bool] = {}
    draft_by_slug: Dict[str, bool] = {}

    desc_by_id: Dict[int, str] = {}
    desc_by_slug: Dict[str, str] = {}

    title_by_id: Dict[int, str] = {}
    title_by_slug: Dict[str, str] = {}

    cats_by_id: Dict[int, List[str]] = {}
    cats_by_slug: Dict[str, List[str]] = {}

    def parse_dt(s: Optional[str]) -> Optional[datetime]:
        if not s or str(s).strip() == "" or str(s).lower() == "nan":
            return None
        try:
            return dateparser.parse(str(s))
        except Exception:
            return None

    def parse_bool(v: Optional[str]) -> Optional[bool]:
        if v is None:
            return None
        s = str(v).strip().lower()
        if s in ("true", "1", "yes", "y"):
            return True
        if s in ("false", "0", "no", "n"):
            return False
        return None

    import csv as _csv
    with csv_path.open("r", encoding="utf-8", errors="ignore") as f:
        reader = _csv.DictReader(f)
        rows = [{(k.lower() if k else k): v for k, v in row.items()} for row in reader]

    for row in rows:
        pid_raw = row.get("post_id") or ""
        m = re.match(r"^(\d+)(?:\.(.*))?$", str(pid_raw))
        pid = int(m.group(1)) if m else None
        slug = slugify(m.group(2)) if (m and m.group(2)) else None

        dt = (parse_dt(row.get("post_date"))
              or parse_dt(row.get("email_sent_at"))
              or parse_dt(row.get("inbox_sent_at"))
              or parse_dt(row.get("published_at"))
              or parse_dt(row.get("publish_date"))
              or parse_dt(row.get("created_at"))
              or parse_dt(row.get("sent_at")))

        ip = parse_bool(row.get("is_published"))
        draft_val = (None if ip is None else (not ip))

        desc = (row.get("subtitle") or "").strip() or None
        t = (row.get("title") or "").strip() or None

        categories: List[str] = []
        if row.get("type"):
            categories.append(str(row.get("type")).strip())

        if pid is not None:
            if dt: dates_by_id[pid] = dt
            if draft_val is not None: draft_by_id[pid] = draft_val
            if desc: desc_by_id[pid] = desc
            if t: title_by_id[pid] = t
            if categories: cats_by_id[pid] = categories
        if slug:
            if dt: dates_by_slug[slug] = dt
            if draft_val is not None: draft_by_slug[slug] = draft_val
            if desc: desc_by_slug[slug] = desc
            if t: title_by_slug[slug] = t
            if categories: cats_by_slug[slug] = categories

    return (dates_by_id, dates_by_slug,
            draft_by_id, draft_by_slug,
            desc_by_id,  desc_by_slug,
            title_by_id, title_by_slug,
            cats_by_id,  cats_by_slug)

def pick_from_maps(pid: Optional[int], slug: Optional[str], id_map, slug_map, fallback=None):
    if pid is not None and pid in id_map:
        return id_map[pid]
    if slug and slug in slug_map:
        return slug_map[slug]
    return fallback

# -----------------------------
# Main conversion
# -----------------------------

def convert_html_post(html_path: Path, out_root: Path, section: str,
                      maps,
                      dry_run: bool=False) -> None:
    (dates_by_id, dates_by_slug,
     draft_by_id, draft_by_slug,
     desc_by_id, desc_by_slug,
     title_by_id, title_by_slug,
     cats_by_id,  cats_by_slug) = maps

    html = html_path.read_text(encoding="utf-8", errors="ignore")
    soup = BeautifulSoup(html, "html.parser")

    pid, slug_from_name = split_filename_parts(html_path)

    # Title selection
    title_csv = pick_from_maps(pid, slug_from_name, title_by_id, title_by_slug, fallback=None)
    title_html = extract_title_from_html(soup)
    if title_csv and title_csv.strip():
        title = title_csv.strip()
    elif title_html and title_html.strip():
        title = title_html.strip()
    else:
        title = slug_from_name or (html_path.stem)

    tags = extract_tags(soup)
    article_root = find_article_root(soup)
    imgs = collect_images(article_root, html_path)
    og_image = extract_og_image(soup)

    html_fallback = extract_date_from_html(soup, fallback=datetime.fromtimestamp(html_path.stat().st_mtime))
    date_csv = pick_from_maps(pid, slug_from_name, dates_by_id, dates_by_slug, fallback=html_fallback)
    draft_csv = pick_from_maps(pid, slug_from_name, draft_by_id, draft_by_slug, fallback=None)
    desc_csv = pick_from_maps(pid, slug_from_name, desc_by_id, desc_by_slug, fallback=None)
    cats_csv = pick_from_maps(pid, slug_from_name, cats_by_id, cats_by_slug, fallback=[])

    # Slug
    base_slug = slug_from_name or slugify(title)

    # Destination dir: out_root/section/YYYY/MM/DD-slug/
    yyyy = f"{date_csv.year:04d}"
    mm = f"{date_csv.month:02d}"
    dd = f"{date_csv.day:02d}"
    folder_name = f"{dd}-{base_slug}"
    bundle_dir = out_root / section / yyyy / mm / folder_name
    index_md = bundle_dir / "index.md"

    print(f"[INFO] Converting: {html_path.name} → {bundle_dir}")
    if not dry_run:
        bundle_dir.mkdir(parents=True, exist_ok=True)

    # Convert article → markdown
    article_html = str(article_root)
    markdown = html_to_markdown(article_html)

    # Prepare HTTP session for downloads
    session = None if dry_run else build_http_session()

    # Track used file names inside this bundle
    used_names: set[str] = set()

    # 1) Handle local images: copy to bundle and rewrite paths in markdown
    for orig_src, abs_path in imgs:
        if abs_path is None:
            continue  # skip here (handled in external loop if http/https); unresolved locals ignored
        dest_name = Path(orig_src).name
        if dest_name in used_names:
            # de-dup local names by suffixing
            base, ext = os.path.splitext(dest_name)
            i = 2
            candidate = dest_name
            while candidate in used_names:
                candidate = f"{base}-{i}{ext}"
                i += 1
            dest_name = candidate
        used_names.add(dest_name)
        dest_path = bundle_dir / dest_name
        if not dry_run:
            try:
                shutil.copy2(abs_path, dest_path)
            except Exception as e:
                print(f"[WARN] Could not copy image {abs_path} → {dest_path}: {e}")
                continue
        # replace original path (could contain dirs) with ./dest_name
        pattern = re.escape(orig_src)
        markdown = re.sub(pattern, f"./{dest_name}", markdown)

    # 2) Handle external images: download and rewrite
    counter = 1
    for orig_src, abs_path in imgs:
        if not (orig_src.startswith("http://") or orig_src.startswith("https://")):
            continue
        if dry_run:
            # only rewrite to a hypothetical local name to preview
            dest_name = pick_filename_from_url(orig_src, None, f"image{counter}", used_names)
            counter += 1
            pattern = re.escape(orig_src)
            markdown = re.sub(pattern, f"./{dest_name}", markdown)
            continue

        # Download
        try:
            resp = session.get(orig_src, timeout=20, stream=True)
            ctype = resp.headers.get("Content-Type", "")
            dest_name = pick_filename_from_url(orig_src, ctype, f"image{counter}", used_names)
            counter += 1
            dest_path = bundle_dir / dest_name
            if resp.ok:
                with open(dest_path, "wb") as f:
                    for chunk in resp.iter_content(chunk_size=8192):
                        if chunk:
                            f.write(chunk)
                # rewrite references in markdown
                pattern = re.escape(orig_src)
                markdown = re.sub(pattern, f"./{dest_name}", markdown)
            else:
                print(f"[WARN] Failed to download {orig_src} (status {resp.status_code})")
        except Exception as e:
            print(f"[WARN] Error downloading {orig_src}: {e}")

    # featured_image: if external and not already downloaded, fetch it too
    featured_image_local = None
    if og_image:
        if og_image.startswith(("http://", "https://")):
            if dry_run:
                fname = pick_filename_from_url(og_image, None, "featured", used_names)
                featured_image_local = f"./{fname}"
            else:
                try:
                    resp = session.get(og_image, timeout=20, stream=True)
                    ctype = resp.headers.get("Content-Type", "")
                    fname = pick_filename_from_url(og_image, ctype, "featured", used_names)
                    if resp.ok:
                        with open(bundle_dir / fname, "wb") as f:
                            for chunk in resp.iter_content(chunk_size=8192):
                                if chunk:
                                    f.write(chunk)
                        featured_image_local = f"./{fname}"
                    else:
                        print(f"[WARN] Failed to download og:image {og_image} (status {resp.status_code})")
                except Exception as e:
                    print(f"[WARN] Error downloading og:image {og_image}: {e}")
        else:
            # if it's already a local path (shouldn't happen for og:image), just use it
            featured_image_local = og_image if og_image.startswith("./") else f"./{Path(og_image).name}"

    fm = write_front_matter(
        title=title,
        date=date_csv,
        slug=base_slug,
        draft=draft_csv,
        description=desc_csv,
        tags=tags,
        categories=cats_csv,
        featured_image=featured_image_local
    )
    content = fm + "\n" + markdown

    if dry_run:
        print(f"[DRY-RUN] Would write: {index_md}")
    else:
        index_md.write_text(content, encoding="utf-8")
        print(f"[OK] Wrote {index_md}")

def main():
    parser = argparse.ArgumentParser(description="Convert Substack export to Hugo page bundles (CSV-aware v6, downloads external images).")
    parser.add_argument("--export", required=True, help="Path to Substack export ZIP or folder")
    parser.add_argument("--out", default="content", help="Output Hugo content directory (default: content)")
    parser.add_argument("--section", default="posts", help="Hugo section (default: posts)")
    parser.add_argument("--csv", default=None, help="Path to posts.csv (optional; will auto-detect if omitted)")
    parser.add_argument("--dry-run", action="store_true", help="Preview actions without writing files")
    args = parser.parse_args()

    export_path = Path(args.export).expanduser().resolve()
    out_root = Path(args.out).expanduser().resolve()
    section = args.section.strip("/")

    temp_dir = None
    try:
        if export_path.is_file() and export_path.suffix.lower() == ".zip":
            temp_dir = Path(tempfile.mkdtemp(prefix="substack_export_"))
            print(f"[INFO] Unzipping export to {temp_dir} ...")
            with zipfile.ZipFile(export_path, "r") as z:
                z.extractall(temp_dir)
            root = temp_dir
        else:
            root = export_path

        # CSV
        csv_path = Path(args.csv).expanduser().resolve() if args.csv else autodetect_csv(root)
        if csv_path and csv_path.exists():
            print(f"[INFO] Using CSV: {csv_path}")
            maps = build_maps_from_csv(csv_path)
        else:
            print("[WARN] No CSV provided/found. Title/date/draft/description/categories will rely on HTML/fallbacks.")
            maps = ({}, {}, {}, {}, {}, {}, {}, {}, {}, {})  # maintain structure

        posts_dir = find_posts_dir(root)
        html_files = sorted(posts_dir.rglob("*.html"))
        if not html_files:
            print(f"[ERROR] No HTML files found under {posts_dir}")
            return

        print(f"[INFO] Found {len(html_files)} HTML posts under {posts_dir}")
        for html_path in html_files:
            convert_html_post(html_path, out_root, section, maps, dry_run=args.dry_run)

        print("[DONE] Conversion complete.")
        if args.dry_run:
            print("[NOTE] Dry-run only. Re-run without --dry-run to write files.")
    finally:
        if temp_dir and temp_dir.exists():
            shutil.rmtree(temp_dir, ignore_errors=True)

if __name__ == "__main__":
    main()
