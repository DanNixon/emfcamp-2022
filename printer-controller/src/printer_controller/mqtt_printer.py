from threading import Lock
from escpos.escpos import Escpos
import paho.mqtt.client as mqtt
import logging


class MqttPrinter(Escpos):

    def __init__(
        self,
        broker,
        port,
        username,
        password,
        transmit_topic,
        receive_topic,
        receive_control_topic,
        *args,
        **kwargs,
    ):
        Escpos.__init__(self, *args, **kwargs)

        self.transmit_topic = transmit_topic
        self.receive_topic =receive_topic
        self.receive_control_topic = receive_control_topic

        self.client = mqtt.Client()
        self.client.username_pw_set(username, password)
        self.client.connect(broker, port)
        self.client.loop_start()
        self.client.on_connect = self.on_connect
        self.client.on_message = self.on_message

        self.rx_lock = Lock()
        self.rx_lock.acquire()

    def open(self):
        pass

    def _raw(self, msg):
        self.client.publish(
            self.transmit_topic,
            msg,
            2,
        ).wait_for_publish()

    def _read(self):
        self.client.publish(
            self.receive_control_topic,
            "16",
            2,
        ).wait_for_publish()
        self.rx_lock.acquire()
        return self.payload

    def close(self):
        pass

    def on_connect(self, c, o, f, r):
        logging.info("Printer connected to MQTT broker")
        c.subscribe(self.receive_topic)

    def on_message(self, c, o, msg):
        self.payload = msg.payload
        self.rx_lock.release()
