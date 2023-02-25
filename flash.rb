# require the 'curses' library to access curses functionality
require 'cursed'

# set up curses and the screen
Cursed.start do
  Cursed.init_screen
  Cursed.curs_set(0) # hide the cursor

  # infinite loop to flash the screen
  loop do
    Cursed.attrset(Cursed::A_STANDOUT) # set the screen to standout mode
    Cursed.addstr("Flashing screen!") # write to the screen
    Cursed.refresh # update the screen
    sleep(0.5) # wait for half a second

    Cursed.attrset(Cursed::A_NORMAL) # set the screen to normal mode
    Cursed.addstr("Flashing screen!") # write to the screen
    Cursed.refresh # update the screen
    sleep(0.5) # wait for half a second
  end
end
