import * as cdk from "aws-cdk-lib";
import { Construct } from "constructs";
import * as rust from "@cdklabs/aws-lambda-rust";

export class SemSearchStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const dbSecret = new cdk.aws_secretsmanager.Secret(
      this,
      "movies-db-secret"
    );

    const moviesLambda = new rust.RustFunction(this, "MoviesLambda", {
      entry: "backend/movies",
      binaryName: "movies",
      environment: {
        DATABASE_SECRET_NAME: dbSecret.secretName,
      }
    });

    dbSecret.grantRead(moviesLambda);

    moviesLambda.addToRolePolicy(new cdk.aws_iam.PolicyStatement({
      effect: cdk.aws_iam.Effect.ALLOW,
      actions: [
        'bedrock:InvokeModel'
      ],
      resources: [
        `arn:aws:bedrock:${props?.env?.region || 'eu-central-1'}::foundation-model/amazon.titan-embed-text-v2*`
      ]
    }));


    const httpApi = new cdk.aws_apigatewayv2.HttpApi(this, "MoviesApi");

    const moviesLambdaIntegration =
      new cdk.aws_apigatewayv2_integrations.HttpLambdaIntegration(
        "MoviesLambdaIntegration",
        moviesLambda
      );

    httpApi.addRoutes({
      path: "/movies",
      methods: [cdk.aws_apigatewayv2.HttpMethod.GET],
      integration: moviesLambdaIntegration,
    });
  }
}
