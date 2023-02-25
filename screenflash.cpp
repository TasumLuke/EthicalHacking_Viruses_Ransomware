#include <curses.h> // include the curses library
#include <unistd.h> // for the sleep function

int main() {
  // set up curses and the screen
  initscr();
  curs_set(0); // hide the cursor

  // infinite loop to flash the screen
  while (true) {
    attron(A_STANDOUT); // set the screen to standout mode
    printw("Flashing screen!"); // write to the screen
    refresh(); // update the screen
    sleep(0.5); // wait for half a second

    attroff(A_STANDOUT); // set the screen to normal mode
    printw("Flashing screen!"); // write to the screen
    refresh(); // update the screen
    sleep(0.5); // wait for half a second
  }

  endwin(); // clean up curses

  return 0;
}
