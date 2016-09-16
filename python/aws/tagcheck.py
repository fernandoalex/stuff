#
#
# Class used to check information based on the tags of the instance
#
import boto3

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
                        print(i.id, i.tags[0]['Key'], i.tags[0]['Value'])

if __name__ == '__main__':
        ec2_instance = tagChecker()
        ec2_instance.isTimeBetween()
