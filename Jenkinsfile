@Library('snips') _

def branchName = "${env.BRANCH_NAME}"

node('jenkins-slave-ec2') {
    env.PATH = "/usr/local/bin:${env.HOME}/.cargo/bin:${env.PATH}"

    stage('Setup') {
        deleteDir()
        checkout scm
    }

    stage('Build') {
        sh "ssh-agent sh -c 'ssh-add; cargo build --all-features'"
    }

    stage('Test') {
        sh "cargo test --all-features"
    }

    stage('Jar') {
        sh "cd platforms/kotlin && ./gradlew jar"
    }

    switch (branchName) {
        case "develop":
        case "master":
            stage("Upload jar") {
                sh """
                    cd platforms/kotlin
                    ./gradlew uploadArchives -PnexusUsername="$NEXUS_USERNAME" -PnexusPassword="$NEXUS_PASSWORD"
                """
            }
    }

    performReleaseIfNeeded()
}
