#!/usr/bin/env node
import 'source-map-support/register';
import cdk = require('@aws-cdk/core');
import { TideCdkPlayStack } from '../lib/tide-cdk-play-stack';

const app = new cdk.App();
new TideCdkPlayStack(app, 'TideCdkPlayStack');
