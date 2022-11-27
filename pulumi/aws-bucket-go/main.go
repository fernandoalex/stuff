package main

import (
	"github.com/pulumi/pulumi-aws/sdk/v5/go/aws/s3"
	"github.com/pulumi/pulumi/sdk/v3/go/pulumi"
	"math/rand"
	"strconv"
)

func main() {

	// some random number for to avoid colisions with s3 naming
	rand_seed := rand.Int()

	// name for the bucket that we will use
	bucket_name := "mybucket" + strconv.Itoa(rand_seed)

	pulumi.Run(
		func(ctx *pulumi.Context) error {
		// Create an AWS resource (S3 Bucket)
		bucket, err := s3.NewBucket(ctx, bucket_name, nil)
		if err != nil {
			return err
		}

		// Export the name of the bucket
		ctx.Export(bucket_name, bucket.ID())
		return nil
	})
}
