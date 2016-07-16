import boto3

print("Simple function to list all unused ebs volumes in a AWS account")
ec2 = boto3.resource('ec2')

volume_iterator = ec2.volumes.all()

for volume in volume_iterator:
    if volume.state == 'available':
        print [volume.id, volume.availability_zone, volume.state]


