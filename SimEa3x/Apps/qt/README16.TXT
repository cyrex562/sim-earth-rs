Apple's QuickTime Plug-in for the Internet

Version 1.1

Read-me for Windows 3.1 Users

Apple's QuickTime plug-in lets you view QuickTime movies and QuickTime
VR Panoramas and Objects directly in your browser window. This read-me
describes what you will need and how to use the Windows version of
Apple's QuickTime plug-ins for the Internet. If you are a Macintosh
user, please see the Macintosh read-me.

ATTENTION Netscape Navigator Users: This version of the QuickTime
plug-in is newer than the 1.0 version shipping with Netscape Navigator
3.0. Check out the new features listed below!

This document covers:
 
* Installation Instructions 
* New Features 
* Frequent Questions 
* Sources of additional information


Installing the QuickTime Plug-in
---------------------------------
To take full advantage of the QuickTime plug-in, you will need to
install QuickTime for Windows 2.1.2, the plug-in (and optionally the
QuickTime VR software), and configure your browser to use the plug-in.
Here are the details of what you will need and how to configure it.

What you need
-------------
* A 386, 486, or 586/Pentium PC running Windows 3.1
* Memory as required by your operating system
* Netscape Navigator 2.x or 3.x. Note: The plug-in
could not effectively function in the pre-release beta version of
Internet Explorer for Windows 3.1 that we tested with. We will evaluate
a future release when the product is nearer to completion.
* QuickTime 2.1.2 for Windows
* Version 1.1 of the Apple QuickTime Plug-in for the Internet
* Version 1.0.3 or later of the QuickTime VR (Virtual Reality) for
Windows software (QTVRW.QTC).


Installing QuickTime 2.1.2 and the plugin-software
--------------------------------------------------
Please make sure you have QuickTime 2.1.2.xx. Here's how to determine
which version of QuickTime you have:

Open your Control Panels folder (accessible from the "Main" window) and
double-click on the "QuickTime" control panel. The version number will
be displayed in the control panel window. Make sure the control panel is
called "QuickTime", not "QuickTime 32". If you have "QuickTime 32", it
will not work.

If you do not have QuickTime for Windows 2.1.2, you can obtain it from
the QuickTime software page (http://quicktime.apple.com/sw/sw.html). We
recommend you select "Easy Download" to download all the files you will
need, including the plugin and QuickTime Virtual Reality. Here's how to
install the software:

Easy Download will download QTEASY16.EXE, prompting you to specify where
to save it. When the file has downloaded, locate it and double click on
it. This will inflate the files QTINSTAL.EXE, NPQTW16.DLL and QTVRW.QTC
and place them in the directory you downloaded QTEASY16.EXE into (you
may need to refresh the window to see the new files).

Run QTINSTAL.EXE to install QuickTime for Windows on your machine. If
you have older versions of QuickTime files, the installer will ask if
you want to delete them. Please respond "yes" to any such queries.

If you already had QuickTime installed, you can download the plugin
(NPQTW16.DLL) and the QuickTime VR Software (QTVRW.QTC) separately.
Regardless of whether you download them with the "Easy Download" or
individially, you must move these files to the proper locations:

Move the file QTVRW.QTC in the WINDOWS directory of your primary drive.

Move NPQTW16.DLL to the "Plugins" directory in your browser folder. If
you are running a beta version of Internet Explorer 3.0, you will need
to create a folder called "plugins" in the same directory as Internet
Explorer 3.0.

Re-launch your browser.


New Features in Version 1.1
---------------------------
* Expanded Browser Support:  Netscape 3.x and Internet Explorer 3.x 

* QTVR Hot Spots - allows you to click on "hot spots" in a QTVR movie to
jump to a new page.

* Webmaster control over default playback volume for movies.

* "Scaled" movies - movies can play back at different sizes

* "Cached" movies (Netscape 3.0 users only) - movies you have recently
viewed can be cached just like other documents so they don't need to
re-load when you return to them.


Here are some Frequently Asked Questions
----------------------------------------

Q  Why should I install the QuickTime plug-in?

A  Apple's QuickTime plug-in for the Internet lets you experience
QuickTime content directly in the browser window. You no longer need a
"helper application" like MoviePlayer to view QuickTime content over the
internet. You can also view QuickTime VR (Virtual Reality) Panoramas and
Objects right in your browser, after downloading the QuickTime VR
software from Apple.

The QuickTime plug-in works with existing QuickTime movies, and with
movies prepared to take advantage of the plug-in's "fast-start" feature.
The fast-start feature will present the first frame of the movie almost
immediately and can begin playing even before the movie has been
completely downloaded.

Your browser can use the QuickTime plug-in automatically to play any
QuickTime movie while browsing the Internet. Just download the plug-in
to your computer, move it to your browsers' plug-in folder, and
re-launch your browser! The plug-in can play many kinds of QuickTime
movies (.MOV files) including movies with text, MIDI, and other kinds of
data. If you have downloaded and installed the QuickTime VR component,
it will also let you interact with QuickTime VR Panoramas and Objects.


Q I have followed the installation instructions above, but when I get to
a page where a movie is supposed to be, the movie does not appear. There
is no warning or error message - it just seems to ignore the movie.

A  You are probably using Internet Explorer 2.1 for Windows 3.1, which
does not support plugins. Until a final version of Internet Explorer 3.0
for Windows is available, please use Netscape 3.01 as your browser.


Q  When I get to a page where a movie is supposed to be, I get an error
message: "This page uses a plugin which does not seem to be
installed..."

A  Check to see that NPQTW16.DLL is in the plugins folder for your
browser. If you have just placed the NPQTW16.DLL there, you need to quit
and re-start your browser for it to be recognized.

It's also possible that you may need to configure your browser to tell
it to use the plugin. Please refer to the Browser Configuration document
on the web at http://quicktime.apple.com/sw/browserconfig.html for more
information.


Q  Is there any special configuration or set-up I need to do depending on
which browser I am using?

A  Yes; different browsers require different set-up. See the Browser
Configuration document on the web at http://quicktime.apple.com/sw/
browserconfig.html for more information.


Q  When I tried to use the plugin to view a movie, a dialog box appears
complaining that "SHARE" is not loaded. What do I need to do?

A  To make SHARE.EXE load at boot time, edit your AUTOEXEC.BAT startup
file and look for the statement "C:/DOS/SHARE.EXE" (or simply "SHARE").
Add it if it is not there. If is there, but there is a semicolon (";")
or the word "REM" at the beginning of the line, delete the semicolon or
"REM". Save your changes and re-start your computer.


Q  I am using a Web browser other than the ones listed above and I am
experiencing problems with the QuickTime plug-in.

A  In this release the QuickTime for Windows plug-in only supports
Netscape Navigator 3.x for Windows 3.1.


Q  I understand that the Macintosh version of the plug-in supports MPEG.
Why doesn't the PC version support MPEG and how do I download MPEG files
to disk.

A  MPEG support will be added to QuickTime for Windows at a later date.
Meanwhile, there is a variety of third-party software and hardware for
MPEG playback. You may want to consult the MPEG Frequently Asked
Questions Document on the MPEG archive site to find a solution that
meets your needs.


Q  How do I save a QuickTime movie from a Web page to my hard-drive?

A  Just click and hold the right-hand mouse button down over the movie. A
popup will appear that lets you save the movie.


Q  I design web pages and would like to know more about movie controls
and other capabilities of the QuickTime plug-in. Where should I look for
documentation?

A  Please check the QuickTime Webmaster's Page on the web at
http://quicktime.apple.com/sw/webmaster_1.1.html to learn about all of
the capabilities of the Quicktime plug-in.
 

Q  When I click on a QuickTime VR Object or Panorama, I get a blank
window. How do I get VR to work with the QuickTime plug-in?

A  You may not have the most recent version (1.0.3) of the QTVRW.QTC
file in your WINDOWS or WINDOWS\SYSTEM directory. This is the QuickTime
Component file that contains software used by the Quicktime plug-in to
play QuickTime VR Objects and Panoramas, and must be in your WINDOWS or
WINDOWS\SYSTEM directory. Visit the QuickTime Software page to download
QTVRW.QTC.


Q I am browsing a page with a QuickTime VR panorama and expect that I
can use the shift and control keys to zoom in/zoom out of the movie. 
But the movies don't seem to respond to keystrokes as they do when I
play them in the Movie Player.

A  To enable zooming in and out, you need to select the QuickTime VR
panaroma or QuickTime movie first by moving the cursor inside the movie.


Q QuickTime just doesn't seem to be working on my system. Where can I go
to read about other installation issues and troubleshooting techniques?

A  A good source is the technote entitled  "QuickTime For Windows:
Resolving Common Installation Issues". This is available on the web at
http://devworld.apple.com/dev/technotes/tn/tn1074.html.


For More Information
--------------------
Direct your Plug-in feedback and bugs to: qtplug11@apple.com

When reporting a difficulty or problem please be sure to correctly report:
* Which version of the plug-in you are using
* Which version of Windows you are running 
* What the configuration of your machine is (machine type, RAM, etc.)
* What movie or URL location you are at when you experience difficulties

Because of the heavy traffic on the QuickTime pages, we may not be able
to respond to your e-mail individually. We appreciate your feedback!

For general QuickTime support for both Mac and Windows, go to
http://www.info.apple.com where you can find both discussion groups and
FAQs to answer whatever questions you might have.



QuickTime Plug-in Sample Site
-----------------------------

Visit the new QuickTime Plug-in Sample Site at
http://www.mediacity.com/~erweb and see how developers are using
embedded QuickTime media to enhance web sites. Before your visit, make
sure you've already installed the plug-in and related QuickTime software
as described above.


This readme was last modified on December 11, 1996





