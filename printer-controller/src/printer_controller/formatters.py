from datetime import datetime
import logging


def print_event(p, event):
    event_type = event["event"]["type"]

    if event_type in ["talk", "workshop", "youth_workshop", "performance"]:
        p.set(
            align="center",
            underline=1,
        )

        if event_type == "talk":
            p.text("Talk\n")
        elif event_type == "workshop":
            p.text("Workshop\n")
        elif event_type == "youth_workshop":
            p.text("Youth Workshop\n")
        elif event_type == "performance":
            p.text("Performance\n")

        p.text(f"{event['event']['title']}\n")

        p.set(align="center")
        p.text(f"{event['event']['speaker']} ({event['event']['pronouns']})\n")
        p.text(f"{event['venue']}\n")

        p.set()
        p.text(f"{event['event']['description']}")

        p.qr(event["event"]["link"], center=True)

    elif event_type == "film":
        p.set(
            align="center",
            underline=1,
        )
        p.text(f"{event['event']['title']}\n")

        p.set(align="center")
        p.text(f"{event['event']['certificate']} ({event['event']['runtime']})\n")

        p.set()
        p.text(f"{event['event']['description']}")

        p.qr(event["event"]["imdb_url"], center=True)

    else:
        logging.warning(f"Unknown event type: {event_type}")


def print_schedule_announcement(p, data):
    logging.info("Start printing schedule")

    p.set(
        align="center",
        underline=1,
        double_width=True,
        double_height=True,
    )
    p.text("Starting Soon\n")

    p.set(
        align="center",
        double_width=True,
        double_height=True,
    )
    timestamp = datetime.fromisoformat(data[0]["start"])
    timestamp = timestamp.strftime("%A %H:%M")
    p.text(f"{timestamp}\n")

    p.print_and_feed(2)

    for event in data:
        print_event(p, event)

    p.cut()

    logging.info("Printing done")


def print_general_accouncement(p, data):
    logging.info("Start printing announcement")

    p.set(
        align="center",
        underline=1,
        double_width=True,
        double_height=True,
    )
    p.text("Announcement\n")

    p.set(
        align="center",
        double_width=True,
        double_height=True,
    )
    p.text(f"{data['timestamp']}\n")

    p.print_and_feed(2)

    p.set(
        align="center",
        underline=1,
        double_width=True,
        double_height=True,
    )
    p.text(f"{data['title']}\n")
    p.print_and_feed(1)

    p.set()
    p.text(f"{data['msg']}\n")

    if "qr" in data:
        p.qr(data["qr"], center=True)

    p.cut()

    logging.info("Printing done")
