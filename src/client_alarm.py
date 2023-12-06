import os
import sys
import time
import random
import threading
import subprocess
import configparser
import requests
import platform

SLEEPY_TIME = 10  # get an hour of silence, not anymore, but normally
IP = "192.168.231.179"

# Global thread list, if empty, all threads will exit.
threads = []


def main_thread(media) -> int:
    while True:
        if not threads:  # Kill switch
            break

        time.sleep(SLEEPY_TIME)
        # do what ever you want to do

        rand_int = random.randint(0, len(media) - 1)
        # run the mplayer with the media file
        subprocess.run(f"mplayer -ao alsa {media[rand_int]}".split())

    return 0


def main() -> int:
    media = []
    # set default config 
    path = f'{os.environ["HOME"]}/.config/move-alarm/config.cfg'
    windows_path = path.replace('/', '\\')

    if platform.platform == 'windows':
        path = windows_path


    url = f'https://{IP}:6969/api/web/config/Olaf'
    if os.path.isfile(path):  # check if we have an config, if not pull media_path from server
        cfg = configparser.ConfigParser()
        cfg.read(path)
        path = cfg.get("media_location", "path")
    else:
        response = requests.get(url)
        if response.status_code == 200:
            path = response.content

    for i in os.walk(f'{path}'.format(HOME=os.environ["HOME"])):
        if not i[2]:
            continue

        media.extend(i[0] + j for j in i[2])

    # create main thread and append to threads list
    _main_thread = threading.Thread(target=main_thread, args=(media,))
    threads.append(_main_thread)

    # start all threads
    for i in threads:
        i.start()

    return 0


if __name__ == "__main__":
    sys.exit(main())
