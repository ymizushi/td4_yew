rsync -a static/ ../ymizushi.github.io/td4-yew/ --exclude .gitignore
cd ../ymizushi.github.io/td4-yew/
git add .
git commit -m "update content"
git push origin master
