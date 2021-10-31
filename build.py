#!/bin/python3

from zipfile import ZipFile
import json
import os
from distutils.file_util import copy_file
import subprocess
from pathlib import Path

os.system('cargo build --release')
compile_dir = Path('target/release')
copy_file(os.path.join('target', 'release', 'factorio_sc_graphics'), os.path.join('luasrc', 'sc_graphics_extractor'))
subprocess.check_call([os.path.join('target', 'release', 'factorio_sc_graphics'), 'luasrc'])
subprocess.check_call([os.path.join('target', 'release', 'scrubber'), os.path.join('luasrc', 'graphics')])

with open('luasrc/info.json', 'r') as info_file:
    info = json.load(info_file)

build_dir = 'build'
zip_file_name = info['name'] + '_' + info['version'] + '.zip'
os.makedirs(build_dir, exist_ok=True)

with ZipFile(os.path.join(build_dir, zip_file_name), 'w') as fp:
    p = Path('luasrc')
    for filename in p.rglob('*'):

        print(filename)
        fp.write(filename, Path(*filename.parts[1:]))
