Starting SimEarth
-----------------

To start SimEarth:

1.      Run the install program to decompress the program file, set the
default directory, video and sound options.

2.      Change to the drive that contains the directory where you installed
SimEarth.

3.      Change to the directory where you installed SimEarth.

4.      Type Simearth and press the Enter key.

The program will then load and display the SimEarth title screen.  Press
the space bar or mouse button to clear the title screen and you will be
presented with a tutorial window.

If your machine is capable of displaying more than one of the supported
video modes, SimEarth will allow you to switch to a different mode from
the command line.  You will first have to either run the install program
to copy the appropriate files for each video mode or you can copy all the
files from all the data disks to your SimEarth directory.  Remember if you
run the install program, the default settings will be set to whatever was
selected the last time you ran the install program.

Once the you have installed all the video modes, you may use the following
command line switches to override the default options:

Switch  Graphic mode
/dV     Sets VGA 16 colors 640x480
/dE     Sets EGA 16 colors 640x480
/de     Sets EGA 16 colors 320x200
/d2     Sets MCGA 256 colors 320x200
/dT     Sets Tandy 16 colors 320x200
/dM     Sets EGA mono 640x350
/dm     Sets VGA mono 640x480
/dH     Sets Hercules 720x348

Switch  Sound board
/sa     Adlib
/sb     Soundblaster
/sc     Covox
/si     IBM internal
/st     Tandy

For example, to run SimEarth in the EGA lores mode with Adlib sound type:

SIMEARTH /de /sa

and press the Enter key.


Ghost Mouse
-----------

If you run SimEarth using Microsoft's mouse driver version 7.00 in VGA or
EGA Hires modes, Ghost cursor images may appear on your screen.  To correct
this problem you may either run SimEarth using the /B command line option
or order the latest mouse driver from Microsoft.  The /B command line option
may slow down the simulation speed.


Querying in the compressed edit window
-------------------------------------

Because the compressed edit window only displays every other tile, when
you do a query on a tile it may not display all the information.  If you
would like to see all the information, turn off the compress edit window
option.


Amstrad VGA/EGA cards
---------------------

The VGA/EGA display mode on Amstrad PC computers does not support the
640x350 or 640x480 display size.  Please select the 320x200 when using the
install program.

_________________________________________________________________________

Lancement de SimEarth
---------------------

Pour lancer SimEarth:

1.       Ex�cutez le programme Installe pour d�comprimer le fichier
programme, s�lectionnez le r�pertoire syst�matique (par d�faut), les
options vid�o et son.

2.       Passez � l'unit� qui contient le r�pertoire o� vous avez install�
SimEarth.

3.       Passez au r�pertoire o� vous avez install� SimEarth.

4.       Tapez Simearth et pressez la touche Enter.

Le programme se chargera alors et le titre SimEarth s'affichera � l'�cran.
Pressez la barre d'espace ou le bouton de la souris et une fen�tre
d'initiation vous sera pr�sent�e.

Si votre machine est capable d'afficher plus qu'un des modes vid�o pris en
charge, SimEarth vous permettra de passer � un diff�rent mode � partir de la
ligne de commande. Pour cela vous devrez d'abord soit ex�cuter le programme
Installe, pour copier les fichiers ad�quats pour chaque mode vid�o, soit
copier tous les fichiers de tous les disques de donn�es sur votre r�pertoire
SimEarth.
N'oubliez pas que si vous ex�cutez le programme Installe, les param�tres
syst�matiques seront ceux qui ont �t� s�lectionn�s la derni�re fois que vous
avez ex�cut� le programme Installe.

D�s que vous avez install� tous les modes vid�o, vous pouvez vous servir des
options suivantes de la ligne de commande pour annuler les options
syst�matiques:

Option            Mode graphique
/dV               S�lectionne VGA 16 couleurs 640x480
/dE               S�lectionne EGA 16 couleurs 640x480
/de               S�lectionne EGA 16 couleurs 320x200
/d2               S�lectionne MCGA 256 couleurs 320x200
/dM               S�lectionne Tandy 16 couleurs 320x200
/dM               S�lectionne EGA mono 640x350
/dm               S�lectionne VGA mono 640x480
/dH               S�lectionne Hercules 720x348

Option            Carte Son
/sa               Adlib
/sb               Soundblaster
/sc               Covox
/si               IBM interne
/st               Tandy

Par exemple, pour lancer SimEarth au mode EGA Faible R�solution avec Adlib
comme type de son, tapez:

SIMEARTH /de /sa

et pressez la touche Enter.

Double Souris
-------------

Si vous lancez SimEarth en vous servant d'un logiciel pilote de souris
Microsoft version 7.00 aux modes VGA ou EGA Haute R�solution, il se peut que
des images secondaires du curseur apparaissent sur votre �cran. Pour rectifier
ce probl�me, vous pouver soit lancer SimEarth en vous servant de l'option /B
de la ligne de commande, soit commander le dernier logiciel pilote de souris
de Microsoft. L'option /B de la ligne de commande peut ralentir la vitesse de
simulation.

Consultation de la fen�tre �dite condens�e
------------------------------------------

La fen�tre �dite condens�e n'affichant qu'une partie des titres, lorsque vous
la consultez sur un titre, il se peut qu'elle n'affiche pas toute
l'information. Si vous d�sirez voir l'information dans son int�gralit�,
neutralisez l'option fen�tre �dite condens�e.

Cartes Amstrad VGA/EGA
----------------------

Le mode d'affichage VGA/EGA sur les PC Amstrad ne prend pas en charge
l'affichage de dimension 640x350 ou 640x480. S.V.P s�lectionnez 320x200
lorsque vous utilisez le programme Installe.

___________________________________________________________________________

***** Le "CONFIG.SYS" doit inclure les lignes
                            FILES = 10
                            BUFFERS = 10
___________________________________________________________________________

SimEarth starten
----------------

Zum Starten von SimEarth gehen Sie bitte wie folgt vor:

1. Rufen Sie zuerst das Installationsprogramm 'INSTALL.EXE' auf. Es
dekomprimiert die Programmdateien, erstellt das Unterverzeichnis und stellt das
Programm auf Ihre Grafikkarte und die Sound-Karte ein.

2. Schalten Sie nun auf das Laufwerk um, auf das SimEarth kopiert wurde.

3. Danach schalten Sie in das Unterverzeichnis um, in das SimEarth installiert
wurde.

4. Tippen Sie nun bitte 'SIMEARTH', und best�tigen Sie den Befehl mit RETURN
oder ENTER.

Das Programm wird jetzt geladen und pr�sentiert Ihnen als erstes den
Titelbildschirm. Mit der Leertaste oder einer Maustaste gelangen Sie danach zum
Tutorial-Bildschirm.

Falls Ihre Grafikkarte mehr als eine der folgenden Grafikmodi beherrscht,
k�nnen Sie bereits beim Aufrufen von SimEarth auf der Kommandozeile auf einen
anderen Grafikmodus umschalten. Allerdings m�ssen Sie hierf�r vorher die
notwendigen Dateien entwe der mit Hilfe des Installationsprogrammes
installieren oder einfach von den Datendisketten in den SimEarth-Ordner
kopieren. Hinweis: Beim erneuten Aufruf des Installationsprogrammes werden die
Einstellungen des letzten Aufrufes verwendet.

Sobald Sie alle Grafikmodi installiert haben, k�nnen Sie mit folgenden Schalter
beim Programmstart die Standardeinstellung au�er Kraft setzen:

Schalter  Grafikmodus
/dV       VGA mit 16 Farben 640x480
/dE       EGA mit 16 Farben 640x480
/de       EGA mit 16 Farben 320x200
/d2       MCGA mit 256 Farben 320x200
/dT       Tandy mit 16 Farben 320x200
/dM       EGA Monochrom 640x350
/dm       VGA Monochrom 640x480
/dH       Hercules 720x348

Schalter  Sound-Karte
/sa       Adlib
/sb       Soundblaster
/sc       Covox
/si       IBM internal
/st       Tandy

Um SimEarth zum Beispiel mit den Einstellungen 'EGA niedrige Aufl�sung' und
'ADLIB'-Karte zu starten, geben Sie zum Programmstart folgenden Befehl ein:

SIMEARTH /de /sa

und best�tigen mit ENTER oder RETURN.

Mausgeister
-----------

Falls Sie SimEarth zusammen mit dem Microsoft Maustreiber Version 7.0 in VGA-
oder EGA-Modus starten, erscheinen ab und zu wie von Geisterhand Abbildungen
des Mauszeigers auf dem Bildschirm. Dieses Problem k�nnen Sie entweder mit
einer aktuelleren Ve rsion des Maustreibers l�sen oder mit Hilfe des Schalters
'/B' beim Programmstart umgehen. Die Option '/B' verlangsamt das Tempo der
Simulation.

Abfragen im kleinen Arbeitsfenster
----------------------------------

Da im kleinen Arbeitsfenster nur bestimmte Informationen eingeblendet werden,
erhalten Sie bei Abfragen eventuell nicht alle gew�nschten Details angezeigt.
Falls Sie alle verf�gbaren Informationen ben�tigen, schalten Sie bitte die
Option 'compress ed it window' ab.


Amstrad VGA/EGA Karten
----------------------

Die Option VGA/EGA von SimEarth unterst�tzt auf Amstrad PCs nicht die 640x350
oder 640x480 Aufl�sung. W�hlen Sie bitte anstatt dessen im
Installationsprogramm die Aufl�sung 320x200.
______________________________________________________________________________

****  F�gen Sie foigende Zelle in die "CONFIG.SYS" Datel ein:
                            FILES = 10
                            BUFFERS = 10
------------------------------------------------------------------------------
