# STag

A tool to manage spotify songs and tag/search/filter them.

## Commands

 - init: Initializes the required playlist on the server
 - sync: Syncs the local database with the remote playlist, defaults to only
   sync *down*.
 - tag: Let'ts you add tags and metadata to a given song.
 - search: Search by tag etc.

## Usage would be something like

```
stag help

stag tag /style/foo /instrument/bar
# ...play a different song
stag tag /style/hiphop /language/french
#...
stag query hiphop # all tags with substring hiphop
stag query /style # all tags with a /style
