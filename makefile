.PHONY = install
.PHONY = uninstall
.PHONY = update

MAIN = src/client_alarm.py
OTHER = config.cfg media/
PROJECT = move-alarm
SERVICE = move_alarm_client.service

install:
	chmod +x $(MAIN)
	sudo cp $(MAIN) /usr/bin/$(PROJECT)
	sudo cp $(SERVICE) /etc/systemd/system/
	sudo rsync -av $(OTHER) $(HOME)/.config/$(PROJECT)/ --relative
	sudo systemctl start $(SERVICE)

update:
	sudo cp $(MAIN) /usr/bin/move-alarm
	sudo rsync -av $(OTHER) $(HOME)/.config/$(PROJECT)/ --relative

uninstall:
	sudo rm -rf /usr/share/$(PROJECT)/* /usr/bin/move-alarm
