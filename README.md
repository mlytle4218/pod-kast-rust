% POD-KAST(1) Version 0.8.0 | Pod-Kast Documentation

NAME
====

**pod-kast** — starts the command line podcast retrieval program

SYNOPSIS
========

 **pod-kast**

 **pod-kast** \[**-u** | **--update** | **-d** | **--download** | **-h** | **--help**]

DESCRIPTION
===========
Welcome to Pod-kast 0.8.0 written by Marc Lytle and conceived by myself, Chime Hart.

Over many years almost all podcast klients seemed to be lacking an interface which made sense, as well as features I was hoping for. The only one I understood was hpodder, but it is no-longer available. In Pod-kast, since I am totally blind and listen with Speakup, a Linux screen-reader, we took great care to make all functions really obvious by having numbered menus-and-yes having the word "number" before each digit removing any ambiguity.

Here are some conventions for operating the porogram. Presssing 'q' and pressing enter will quit any menu. If the number of results to be listed exceeds more than one screen, press 'n' and enter for the next page. Press 'p' and enter to move back pages. When listing episodes for download or removal from the download queue, multiple episdoes can be chosen. To choose episodes 1 through 5, episode 9, and episode 13 through 16, you can enter 1-5,9,13-16 and press enter and they will all be processed. Press 'q' and hit enter to process the request.

Just before we begin downloading our favorite podcasts, let's go over in detail each of our menu options. 

- Number 1 Add Catagory. Sure makes it easier to separate items by subject. While I only have 2, you may want many more. It will prompt for a full-path where to save.
- Number 2 You can edit your catagories.
- Number 3 Delete a category. It will not allow you to delete a category if any Podcasts are associated with it still.
_ Number 4 You can add a new podcast. It will prompt for an url-and-then which catagory you will store in. Later we will discuss another way of adding items.
- Number 5 You can edit an exsisting podcast by altering any of the fields such as urls, nick-name of podcast, or your chosen catagory.
- Number 6 You can delete an exsisting podcast. This can be handy if you have an actual or similar/ duplicate item, as the program doesn't check.
- Number 7 Choose episodes to download. This will take you to your list of podcasts to which you are subscribed. It will prompt you for an category. If you enter none, it will list all podcasts. Choose a podcast and the availabel episode will be listed. As you navigate through pages, it will mark the episodes as viewed and will not show up the next time you enter a listing of that podcast's episodes. See archive to view previously viewed episodes.
- Number 8 You can begin your downloads.
- Number 9 You can search for podcasts via Apple podcast search. After typing what you are looking for, you will see the results. Then you can type numbers separately or with dashes if consecutive. After that you would hit a q to go to a screen where you will confirm each catagory and download location of items you have chosen. Items not chosen will be ignored.
- Number 10 You can delete items from your download queue.
- Number 11 You can update all podcasts. Certainly creating a cron-job would be most efficient, but this manual way will announce along the way its progress. Also, from a commandline, you can type
pod-kast -u
- Number 12 You can see previously viewed episodes from your list of podcasts.

Options
-------

-u, --update

:   Updates new episodes for all known podcasts

-d, --download

:   Downloads all podcast episodes currently in the download queue

-h, --help *********not working yet

:   Prints help file.

FILES
=====

*~/.pod-kast/pod-kast-config*

:   Program configuration file.

*/.pod-kast/pod-kast.db*

:   Sqlite database file.

BUGS
====

See GitHub Issues: <https://github.com/[owner]/[repo]/issues>

AUTHOR
======

Marc Lytle <mlytle@gmail.com>

