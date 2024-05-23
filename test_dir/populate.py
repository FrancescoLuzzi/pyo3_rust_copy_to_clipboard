import random

num_files = 50
file_len = 10
file_ext = ["exe", "bat", "ps1", "dll", "bpl", "txt"]
num_ext = len(file_ext)
file_ext_indx = 0

for _ in range(num_files):
    file_name = ""
    for _ in range(file_len):
        file_name += chr(random.randrange(97, 122))
    file_name += "." + file_ext[file_ext_indx]
    file_ext_indx = (file_ext_indx + 1) % num_ext
    with open(file_name, "w"):
        pass
