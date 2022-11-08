@Library('jenkins-library')

String agentLabel             = 'docker-build-agent'
String registry               = 'docker.soramitsu.co.jp'
String dockerBuildToolsUserId = 'bot-build-tools-ro'
String dockerRegistryRWUserId = 'bot-sora2-rw'
String cargoAuditImage        = registry + '/build-tools/cargo_audit'
String envImageName           = registry + '/sora2/env:sub4'
String rustcVersion           = 'nightly-2021-12-10'
String wasmReportFile         = 'subwasm_report.json'
String palletListFile         = 'pallet_list.txt'
String appImageName           = 'docker.soramitsu.co.jp/sora2/substrate'
String secretScannerExclusion = '.*Cargo.toml'
Boolean disableSecretScanner  = false
int sudoCheckStatus           = 101
String featureList            = 'private-net include-real-files reduced-pswap-reward-periods'
Map pushTags                  = ['master': 'latest', 'develop': 'dev','substrate-4.0.0': 'sub4']

String contractsPath          = 'ethereum-bridge-contracts'
String contractsEnvFile       = 'env.template'
String solcVersion            = '0.8.14'
String nodeVersion            = '14.16.1'
String gitHubUser             = 'sorabot'
String gitHubRepo             = 'github.com/soramitsu/sora2-substrate.git'
String gitHubBranch           = 'doc'
String gitHubEmail            = 'admin@soramitsu.co.jp'
String cargoDocImage          = 'rust:1.62.0-slim-bullseye'

pipeline {
    options {
        buildDiscarder(logRotator(numToKeepStr: '20'))
        timestamps()
        disableConcurrentBuilds()
    }
    agent {
        label agentLabel
    }
    stages {
        stage('Secret scanner') {
            steps {
                script {
                    gitNotify('main-CI', 'PENDING', 'This commit is being built')
                    docker.withRegistry('https://' + registry, dockerBuildToolsUserId) {
                        secretScanner(disableSecretScanner, secretScannerExclusion)
                    }
                }
            }
        }
        stage('Audit') {
            steps {
                script {
                    docker.withRegistry( 'https://' + registry, dockerBuildToolsUserId) {
                        docker.image(cargoAuditImage + ':latest').inside(){
                            sh '''
                                rm -rf ~/.cargo/.package-cache
                                cargo audit  > cargoAuditReport.txt || exit 0
                            '''
                            archiveArtifacts artifacts: "cargoAuditReport.txt"
                        }
                    }
                }
            }
        }
        stage('Solidity Static Scanner') {
            steps {
                script {
                    docker.withRegistry('https://' + registry, dockerBuildToolsUserId) {
                        slither(contractsPath, contractsEnvFile, solcVersion, nodeVersion)
                    }
                }
            }
        }
        stage('Tests') {
            steps {
                script {
                    docker.withRegistry('https://' + registry, dockerRegistryRWUserId) {
                        docker.image(cargoDocImage).inside(){
                                sh """
                                    cargo test  --release --features runtime-benchmarks
                                """
                        }
                    }
                }
            }
        }
    }
    post {
        always {
            script{
                gitNotify('main-CI', currentBuild.result, currentBuild.result)
            }
        }
        cleanup { cleanWs() }
    }
}
