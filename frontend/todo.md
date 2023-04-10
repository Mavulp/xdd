# To do

- [] Long title does not wrap on new line (should push entire row further to stay inline) 
- [] When scroll up arrow appears, it pushes footer down, footer could be either absolute or the arrow should have height 0
- [] Update language
  - [] Emote is only a category, should call it alias always
  - [] /home -> /emotes
  - [] Add active emote to the URL (open it with it too)

## Done

- [x] Improve creating / editing form to include preview (maybe use modal at the bottom too?)
  - [x] Check if content is url
    - [x] Load content and check its size
      - [x] if it's below 64 (or some threshold), suggest emote (animated or normal based on ending)
      - [x] else just assign gif or image
  - [x] if not url, set content as text
- [x] Update logo colors
- [x] Add 1 hour session timeout for fetching quotes (in the main view only)
  - Cancel fetch if quotes list exists && has been fetched within the next 60 minutes
- [x] User dropdown
  - [x] Log-out button, redirects to a logout page (deletes cookie / local
    storage auth) and logout page will have a "login-in" button that just
    redirects to Routehome because authentication is forced automatically
- [x] Add alias pagination (200 per page)
- [x] Include pagination in the header
- [x] Search & categories scroll with user
- [x] Implement display types
  - [x] Large Icons
  - [x] list type
- [x] Implement edit form
- [x] Correctly display emotes based on type
- [x] Show amount of aliases in search placeholder "Serch in 12512 emotes"
- [x] Clicking emote shows it's detail
  - [x] Contains the full copy pasta / large image etc
  - [x] Information about it (big name, who added it)
  - [x] Big copy button somewhere
  - [x] Edit / delete buttons
