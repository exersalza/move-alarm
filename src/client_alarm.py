import os
import sys
import time
import random
import threading
import subprocess
import configparser

SLEEPY_TIME = 10  # get an hour of silence, not anymore, but normally

threads = []

# 16.4 -> 19.9
def main_thread(media) -> int:
    while True:
        if not threads:  # Kill switch
            break

        time.sleep(SLEEPY_TIME)
        # do what ever you want to do

        rand_int = random.randint(0, len(media) - 1)
        subprocess.run(f"mplayer -ao alsa {media[rand_int]}".split()) 

    return 0


def main() -> int:
    media = []

    cfg = configparser.ConfigParser()
    cfg.read(f'{os.environ["HOME"]}/.config/move-alarm/config.cfg')
    
    media_path = cfg.get("media_location", "path")

    for i in os.walk(f'{media_path}'.format(HOME=os.environ["HOME"])):
        if not i[2]:
            continue
        
        media.extend(i[0] + j for j in i[2])

    _main_thread = threading.Thread(target=main_thread, args=(media,))
    threads.append(_main_thread)

    for i in threads:
        i.start()

    return 0


if __name__ == "__main__":
    sys.exit(main())
