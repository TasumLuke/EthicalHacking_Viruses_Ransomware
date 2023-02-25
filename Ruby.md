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
