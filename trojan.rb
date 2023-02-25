# This is a simple Trojan program that will display a message on the victim's screen when run.

# Require the 'cursed' library to access curses functionality
require 'cursed'

# Set up curses and the screen
Cursed.start do
  Cursed.init_screen
  Cursed.curs_set(0) # Hide the cursor

  Cursed.attrset(Cursed::A_STANDOUT) # Set the screen to standout mode
  Cursed.addstr("You have been hacked!") # Write the message to the screen
  Cursed.refresh # Update the screen
end
