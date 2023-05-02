# Aliases

Hivecom Aliases is an emote management and provider system for other hivecom projects. It allows registered users to create, edit and share emotes which can then be used in other applications which involve user input, such as [hifriends](https://friends.hivecom.net/) or [quotes](https://quotes.hivecom.net/).

We call them `aliases` instead of `emotes` because while in most cases the valiue contains an URL to an image, it is possible to also alias a chunk of text.

---

## Screenshots

### Homepage

The main list of emotes. It allows users to search through, filter on or even change the way the aliases are listed.

![Screenshot of the alias list in dark mode](/screenshots/xdd-mainpage.png "Homepage Dark Mode")

![Screenshot of the alias list light mode](/screenshots/xdd-lightmode.png "Homepage Light Mode")

### Alias Creation

Alias content is provided as a string. Meaning you can paste in a link or write a text. Under the hood the code tries to recognize which category the content falls under but the user can adjust that if needed. So far the categories are mainly used for sorting in the alias list but other services could also use them for other purposes.

The app also supports importing large amounts of aliases in the form of a CSV file. The CSV file must only have two columns, of which the first column is the alias name and second column its content (eg: a url or text content).

![Screenshot of the alias creation page](/screenshots/xdd-create.png "Alias Create Page")

---

The app uses authentication and permission system provided by our own authentication system [idlib](https://github.com/Mavulp/hiveID).
