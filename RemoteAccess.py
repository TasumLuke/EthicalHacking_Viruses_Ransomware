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
