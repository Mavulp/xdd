# Aliases

Hivecom Aliases is an emote management and provider system for other Hivecom projects. It allows registered users to create, edit and share emotes which can then be used in other applications that involve user input such as [hifriends](https://friends.hivecom.net/) or [quotes](https://quotes.hivecom.net/).

We call them `aliases` instead of `emotes` because while in most cases the value contains an URL to an image, it is possible to also alias a chunk of text.

---

## Screenshots

### Homepage

The main list of emotes. It allows users to search through, filter on or even change the way the aliases are listed.

![Screenshot of the alias list in dark mode](/screenshots/xdd-mainpage.png "Homepage Dark Mode")

![Screenshot of the alias list light mode](/screenshots/xdd-lightmode.png "Homepage Light Mode")

### Alias Creation

Alias content is provided as a string. Meaning you can paste in a link or insert text. Under the hood the application tries to recognize which category the content falls under but the user can adjust that if needed. So far the categories are mainly used for sorting in the alias list but other services could also use them for other purposes.

The app also supports importing large amounts of aliases in the form of a CSV file. The CSV file must only have two columns, of which the first column is the alias name and second column its content (eg: a url or text content).

![Screenshot of the alias creation page](/screenshots/xdd-create.png "Alias Create Page")

---

The app uses our own authentication and permission system called [idlib](https://github.com/Mavulp/hiveID).
