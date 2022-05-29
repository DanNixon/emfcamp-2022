from .formatters import print_schedule_announcement
from .mqtt_printer import MqttPrinter

import json
import logging
import paho.mqtt.client as mqtt
import os


def on_connect(c, o, f, r):
    logging.info("Connected to MQTT broker")
    c.subscribe("emfcamp/timely_schedule")


def main():
    logging.basicConfig(level=logging.DEBUG)
    logging.info("Hello!")

    mqtt_password = os.environ["MQTT_PASSWORD"]

    printer = MqttPrinter(
        broker="139.162.198.236",
        port=31883,
        username="printer_publisher",
        password=mqtt_password,
        transmit_topic="thermal_printer/tx",
        receive_topic="thermal_printer/rx",
        receive_control_topic="thermal_printer/rx/control",
        profile="TM-T88IV",
    )

    logging.info(f"Printer online = {printer.is_online()}")
    logging.info(f"Paper status = {printer.paper_status()}")

    def on_schedule_message(m, o, msg):
        logging.info("Got schedule message")
        try:
            data = json.loads(msg.payload)
            print_schedule_announcement(printer, data)
        except Exception as e:
            logging.error(e)

    mqttc = mqtt.Client()
    mqttc.on_connect = on_connect
    mqttc.username_pw_set("printer_publisher", mqtt_password)
    mqttc.connect("139.162.198.236", 31883)
    mqttc.message_callback_add("emfcamp/timely_schedule", on_schedule_message)
    mqttc.loop_forever()
