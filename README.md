# CleanSRT

This application was to solve a simple situation. When collecting a large number of videos, you collect SRT files that have text you do not want in them. I need something to remove that and renumber them.

## Arguments

**-f** or **--file** sets the SRT file or the folder to scan in.
**-o** or ** --output** This is optional. It only works for a individual file. Folders automatically put it in the original srt's folder.
**-d** or **--delete** This flags to delete the old file. By default it keeps it with .OLD. appended to the name.

**Text to filter out** Only one of these two will work at a time.
**-t**  or **--text** Use this for a single instance. Put \n where the line breaks. ie -t "Created by\nSome Subtitle Company"
**-T** or **--text-file** Location of a text file with the text you want removed. In the code is an example using the YTS site tags. Each one you want removed needs to be on a new line. Use \n to break one into multiple lines.
 
## What to Expect
When used for single file, it copies the original to .OLD.srt. Then creates the edited version in the original's place. It will do this regardless if it has a change or not. This is because I made it for scanning thousands of these files.

When using with a folder, it first scans for all SRT files. Once the list is created it rolls through what it found.

## Free

I am starting my IT company. I am trying to raise some funds. If you like the software, please contribute.
Zelle bmoore@tekgnosis.works
venmo @tekgnosis
