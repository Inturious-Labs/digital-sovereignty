# Digital Sovereignty Chronicle - Development Plan

## Pending Tasks

### UI/UX Optimization
- [ ] Strategize newsletter sign-up form placement
  - Description: Optimize the position of the newsletter sign-up form to be immediately visible without waiting for Rapport widget to load
  - Current Issue: The sign-up form appears after Rapport widget, which means it has to wait for Rapport to finish loading before becoming visible
  - Goal: Make the sign-up form appear "in the viewer's face" without any delay for maximum conversion
  - Priority: Medium
  - Considerations:
    - Could move sign-up form before Rapport widget
    - Could use sticky positioning or inline placement within content
    - Need to balance user experience with conversion goals

### Content Cleanup
- [ ] Remove Substack blurbs from articles
  - Description: Remove "Digital Sovereignty Chronicle is a reader-supported publication..." promotional text from article content
  - Affected articles: Most articles from January-March 2025
  - Priority: Low
  - Notes: These blurbs were leftover from the Substack migration

## Completed Tasks
- [x] Integrate Rapport comment system
  - Added Rapport widget to single.html template
  - Added `enable_rapport: true` flag to all published articles
