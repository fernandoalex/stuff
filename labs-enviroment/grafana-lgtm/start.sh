#!/usr/bin/env bash

# print "starting minikube"
# minikube start --driver=docker

kubectl delete configmap -n monitoring loki-config
kubectl delete configmap -n monitoring grafana-datasource

kubectl create configmap -n monitoring loki-config --from-file=loki-config.yaml
kubectl create configmap -n monitoring grafana-datasource --from-file=grafana-datasource.yaml

kubectl apply -f loki-service.yaml
kubectl apply -f grafana-service.yaml

kubectl apply -f loki-deployment.yaml
kubectl apply -f grafana-deployment.yaml

kubectl create namespace monitoring
helm install --namespace monitoring alloy grafana/alloy
kubectl apply -f alloy-podlogs.yaml
helm upgrade --namespace monitoring alloy grafana/alloy -f alloy-values.yaml
