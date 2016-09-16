#
#
# Class used to check information based on the tags of the instance
#
# Dependecies: pip3 install croniter

import boto3
import datetime

class tagChecker:

        ec2 = boto3.resource('ec2')

        def __init__(self):
                pass

        def listTag(self):
                ec2instances = tagChecker.ec2.instances.all()
                for i in ec2instances:
                        print(i.id, i.tags)

        # Check based on some tag if we are between time A and B
        # this can be used to stop and start the instance
        def isTimeBetween(self):
                ec2instances = tagChecker.ec2.instances.all()
                for i in ec2instances:
                        for y in range(len(i.tags)):
                                if i.tags[y]['Key'] == "start":
                                        start_time = i.tags[y]['Value']
                                if i.tags[y]['Key'] == "stop":
                                        stop_time = i.tags[y]['Value']
                        timeA = datetime.datetime.strptime(start_time, "%H:%M")
                        timeB = datetime.datetime.strptime(stop_time, "%H:%M")
                        now = datetime.datetime.now()
                        now_time = now.time()
                        if now_time >= timeA.time() and now_time <= timeB.time():
                                return True
                        else:
                                return False
if __name__ == '__main__':
        ec2_instance = tagChecker()
        ec2_instance.isTimeBetween()