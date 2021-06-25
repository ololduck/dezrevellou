let dezrevellou = null;

class ArticleCommentAuthor {
    name: string;
    website: string;
    email: string;

    constructor(other) {
        this.name = other.name;
        this.website = other.website;
        this.email = other.email;
    }
}

class ArticleComment {
    uid: number;
    comment: string;
    on: string;
    created: number;
    updated: number;
    author: ArticleCommentAuthor;

    constructor(other) {
        this.uid = other.uid;
        this.comment = other.comment;
        this.on = other.on;
        this.created = other.created;
        this.updated = other.updated;
        this.author = new ArticleCommentAuthor(other.author);
    }

    createNode(): Element {
        let root = document.createElement("div");
        root.classList.add("dezrevellou-comment");
        let cAuthor = document.createElement("span");
        cAuthor.classList.add("dezrevellou-comment-author");
        if (this.author.website) {
            let cAuthorWebsiteLink = document.createElement("a");
            cAuthorWebsiteLink.setAttribute("href", this.author.website || '#');
            cAuthorWebsiteLink.innerText = this.author.name || "Anonyme";
            cAuthor.appendChild(cAuthorWebsiteLink);
        } else {
            cAuthor.innerText = this.author.name || "Anonyme";
        }
        root.appendChild(cAuthor);
        let cCreated = document.createElement("span");
        cCreated.classList.add("dezrevellou-comment-created");
        cCreated.innerText = new Date(this.created).toDateString();
        root.appendChild(cCreated);
        let cText = document.createElement("div");
        cText.classList.add("dezrevellou-comment-text");
        cText.innerText = this.comment;
        root.appendChild(cText);
        return root;
    }
}

class Dezrevellou {
    apiUrl: string;
    slug: string;
    comments: ArticleComment[];

    createNewCommentForm() {
        return `<div id="dezrevellou-comment-form">
<textarea name="dezrevellou-comment" placeholder="Votre commentaire ici..."></textarea>
<input class="dezrevellou-author" type="text" name="dezrevellou-author-name" placeholder="nom (optionnel)" />
<input class="dezrevellou-author" type="url" name="dezrevellou-author-website" placeholder="site web (optionnel)" />
<input class="dezrevellou-author" type="email" name="dezrevellou-author-email" placeholder="email(optionnel;non-partagé)"/>
<button id="dezrevellou-submit" type="button"
    onclick="return dezrevellou.postNewComment();">Envoyer</button>
</div>`;
    }

    getAndRenderComments() {
        fetch(this.apiUrl + "/comments/" + this.slug)
            .then(resp => resp.json())
            .then(data => data.map(e => new ArticleComment(e)))
            .then(comments => {
                this.comments = comments;
                const comment_list = document.getElementById("dezrevellou-comments-list");
                comment_list.innerText = '';
                if (this.comments.length > 0) {
                    this.comments.forEach(c => comment_list.appendChild(c.createNode()));
                } else {
                    comment_list.innerHTML = "<p>Pas de commentaires pour l'instant…</p>"
                }
            })
            .catch(err => console.error(err));
    }

    constructor(apiUrl: string, slug: string, elementSelector: string) {
        this.apiUrl = apiUrl;
        this.slug = slug;
        this.comments = [];
        dezrevellou = this;
        let comment_root = document.getElementById(elementSelector);
        // create the input for comment posting
        comment_root.innerHTML = this.createNewCommentForm() + "<hr />";
        //get comments and append them to the element
        let comment_list = document.createElement("div");
        comment_list.id = "dezrevellou-comments-list";
        comment_root.appendChild(comment_list);
        this.getAndRenderComments();
        comment_root.innerHTML += this.signature();
    }

    signature() {
        return `<hr/><p id="dezrevellou-signature"><em>powered by <a href="https://github.com/paulollivier/dezrevellou">dezrevelloù</a>.</em></p>`
    }

    postNewComment() {
        const c = new ArticleComment({
            uid: null,
            comment: (<HTMLInputElement>document.getElementsByName("dezrevellou-comment")[0]).value,
            on: this.slug,
            author: {
                name: (<HTMLInputElement>document.getElementsByName("dezrevellou-author-name")[0]).value,
                website: (<HTMLInputElement>document.getElementsByName("dezrevellou-author-website")[0]).value,
                email: (<HTMLInputElement>document.getElementsByName("dezrevellou-author-email")[0]).value,
            }
        });
        fetch(this.apiUrl + "/comments/" + this.slug, {
            method: "POST",
            headers: {
                'Content-Type': 'application/json;charset=UTF-8'
            },
            body: JSON.stringify(c)
        }).then(_e => console.log("sent new comment")).then(() => this.getAndRenderComments()).catch(e => console.error(e));
    }
}
