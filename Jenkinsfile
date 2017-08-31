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

    if (branchName.startsWith("release/")) {
        stage('release') {
            deleteDir()

            def split = branchName.split('/')

            if (split.length != 2) {
                error("release branch name not formatted correctly")
            }
            def newVersion = split[1]
            def components = newVersion.split('-')
            if (components.length > 2) {
                error("new version doesn't look like semver : component.length = ${components.length}")
            }
            def mainVersion = components[0].split("\\.")
            if (mainVersion.length != 3) {
                error("new version doesn't look like semver : mainVersion.length = ${mainVersion.length} , mainVersion = $mainVersion")
            }
            def nextVersion
            if (components.length == 2) {
                nextVersion = "${components[0]}-SNAPSHOT"
            } else {
                nextVersion = "${mainVersion[0]}.${(mainVersion[1] as Integer) + 1}.0-SNAPSHOT"
            }

            sh "git config --global user.email 'tobor.spins@snips.net'"
            sh "git config --global user.name 'Tobor'"
            sh "git clone git@github.com:snipsco/hermes-protocol.git ."

            sh "git checkout $branchName"
            sh "./update_version.sh $newVersion"
            sh "git commit -am \"set the version to $newVersion\""
            sh "git checkout master"
            sh "git merge --no-ff -m $newVersion $branchName"
            sh "git tag $newVersion"
            sh "git checkout $branchName"
            sh "git merge --no-ff -m \"post release $newVersion\" master"
            sh "./update_version.sh $nextVersion"
            sh "git commit -am \"set the version to $nextVersion\""
            sh "git checkout develop"
            sh "git merge --no-ff -m \"new development cycle : $nextVersion\" $branchName"
            sh "git push origin --tags && git push origin master && git push origin develop && git push origin :$branchName"
        }
    } else {
        echo "not on a release branch, skipping the release !"
    }

}
