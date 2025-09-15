{{/* Generate a name for the chart */}}
{{- define "actix-devops-web.name" -}}
{{- default .Chart.Name .Values.nameOverride | trunc 63 | trimSuffix "-" -}}
{{- end }}

{{/* Generate a full name */}}
{{- define "actix-devops-web.fullname" -}}
{{- $name := include "actix-devops-web.name" . -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s" $name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end }}
