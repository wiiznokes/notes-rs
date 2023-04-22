# Notes

## docs
- https://doc.rust-lang.org/book/ch00-00-introduction.html
- https://docs.rs/iced/latest/iced/
- https://www.youtube.com/watch?v=TJTDTyNdJdY
## repo
- https://github.com/iced-rs/iced
- https://github.com/pop-os/cosmic-epoch
- https://github.com/pop-os/cosmic-text
- https://github.com/pop-os/cosmic-text-editor
- https://github.com/iced-rs/iced_aw


## example
- https://github.com/iced-rs/iced/tree/master/examples/pane_grid
- https://github.com/iced-rs/iced/tree/master/examples/todos

## example files explo
- https://github.com/Kaiden42/PWDuck

## search engine
- https://github.com/tsoding/seroost

## Steps

- definir la stucture qui contient les fichier dans app.rs
- reussir a afficher la structure principale du projet
```
column {
    row {
        actions
    }
    row {
        files_view
        onglets
    }
}
```
- envoyer un message a partir de app.rs
- envoyer un message a partir d'action.rs
- modifier le state de l'app a partir d'action.rs
- implementer la structure de fichier
- reflechir a comment update la view a chaque modification d'un fichier autre que par l'app
- apprendre à gerer la concurency en Rust


## Dependencies Fedora 

- for iced
`sudo dnf install freetype-devel expat-devel fontconfig-devel`

- for cosmic (not used in here)
`sudo dnf install rust-gdk-sys+default-devel`


## fonctionnalité

- file type support plan: pdf, md, (html, CSS), plain text
- tabs
- modif
- view .txt et .md
- params
- fetch
- send
- create files
- create dirs
- dirs and files navigation
- select racine
- search engine


## idée

majeurs fonctionnalitées:
- search
- view tree (must be reactive)
- display (must be reactive, the display content must be cached)


Pour view tree:

Je veux faire un application en Rust qui afficher un repertoire sous forme d'arbre.
Pense tu que ceci est une bonne approche pour ma structure de donnée ?

struct dir_node {
    path: Path,
    is_expand: bool,
    full_name: String,

    content: HashMap<string, soit dir_node soit file_node>
}



struct file_node {
    extension: String,
    full_name: String,
    path: Path
}