# Hivecom Emotes (codename: xdd)

Is an emote management and provider system for other hivecom projects. It allows registered users to create, edit and share emotes which can then be used in other applications which involve user input. Such as [hifriends](https://friends.hivecom.net/) or [quotes](https://quotes.hivecom.net/).

The app uses authentication and permission system provided by our own authentication system [idlib](https://github.com/Mavulp/hiveID).

Generally, we call the emote as `alias` because while in most cases it is an image, it is possible to also alias a text. This wider range of usage calls for a better term than `emote` as that is limited to images / gifs.

---

## Screenshots

### Homepage

The main list of emotes. It allows users to search through, filter on or even change the way alias list is displayed.

![Screenshot of the alias list in dark mode](/screenshots/xdd-mainpage.png "Homepage Dark Mode")

![Screenshot of the alias list light mode](/screenshots/xdd-lightmode.png "Homepage Light Mode")

### Alias Creation

Alias content is provided as a string, meaning you can paste in a link or write a text. Under the hood the code is able to recognize under which category the content falls but the user can also specify that themselves. The categories are mainly used for sorting in the alias list.

The app also supports importing large amounts of aliases in the form of a CSV file. The CSV file must only have two columns, of which the first column is the alias name and second column its content (eg: a url or text content).

![Screenshot of the alias creation page](/screenshots/xdd-create.png "Alias Create Page")
