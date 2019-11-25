import { expect as expectCDK, matchTemplate, MatchStyle } from '@aws-cdk/assert';
import cdk = require('@aws-cdk/core');
import TidePlayCdk = require('../lib/tide-cdk-play-stack');

test('Empty Stack', () => {
    const app = new cdk.App();
    // WHEN
    const stack = new TidePlayCdk.TideCdkPlayStack(app, 'MyTestStack');
    // THEN
    expectCDK(stack).to(matchTemplate({
      "Resources": {}
    }, MatchStyle.EXACT))
});
