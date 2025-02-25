# Obsidian Notifications

This project was just for me to have a way to give myself reminders from my Obsidian notes, but it would be a great starting point for anyone who wants to have automated messages from Markdown notes.

This does not tie into Obsidian in any way, you can use this with any markdown note system. Obsidian was just what I was using when I made this.
I use [syncthing](https://syncthing.net) to keep my notes on all my devices and the sync conflict notifications are based on syncthing so it won't work for just any syncing solution.

Time reminders look for for any bullet points or check boxes that have a time later in the line. Any reminders that start with a check box will not go off if the check mark is checked. Times must be in the format of `(@2025-01-30 2205)` date can be omitted to have a reminder that just goes off every day at the given time `(@ 0725)`.

## Basic Time stamps

This is titled ***Basic*** Time stamps because there are plans for more advanced ways of specifying time in the future, such as cron tab style, and I plan on still supporting the basic version.

TOD = time of day

- time must be in parentheses and start with an `@`  e.g. `(@DATE TIME)`
- TOD must be in 24 hour time
- `:` between hour, minute, and seconds portions is allowed but not required, any other character is not allowed
- TOD can use 1 or 2 digits for hour but must use 2 digits for minute and seconds
- TOD does not need to include seconds, `00` will be assumed for seconds
- TOD must include hour and minute or TOD can be omitted entirely, you can not have just hour
- TOD must have a space (`` ` ` ``) before it even if there is no date `(@YYYY-mm-dd HH:MM)` or `(@ HH:MM)`
- date must not have a space between it and the `@`
- the space condition before year or time will never change, this is how the time written as 2025 is differentiated between 8:25PM and the year 2025
- date can use either 2 or 4 digits for the year, if using 2 the current first 2 digits of the current year are assumed (using only 2 digits to specify a year within the first few minutes of a new century may cause issues)
- date must include all 3 parts (year month day) or be omitted entirely, just specifying a day or month and day or have reminders that repeat every month or year is not currently supported

## Planed Features

- [ ] fix daily reminders only sending 1 message
- [x] current solution for commands is deprecated and should be updated
- [ ] make it so if assumed value for a time has already passed then it will increment the value to get the next time it could happen (I might already be doing this but I should double check that I did)
- [ ] have cronjob style reminders more more advanced repeating settings
- [ ] make reminders for headers
  - so a note `# TODO (@ 0900)` would give me a reminder for all the items under the `TODO` header
  - this should only include top level entries, i.e. nothing that is indented
- [ ] add auto uncheck feature so a reminder can be set to go off every x minutes until it is checked off but still go off again the next day
- [ ] look into alternative options for keeping track of reminders
  - [ ] come up with a unique key system to use instead of basing everything of contents and location
  - [ ] instead of having a separate thread for each timer just keep one and check when the last time each timer was triggered
- [ ] make ignore files and ignore paths configurable instead of just being a hard coded regex string
