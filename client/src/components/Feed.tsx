import { useState } from 'react';
import { useSocial } from '../hooks';

export function Feed() {
  const { posts, createPost, connected, likePost, requestFriend, peerId } = useSocial();
  const [content, setContent] = useState('');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (content.trim()) {
      createPost(content.trim());
      setContent('');
    }
  };

  if (!connected) {
    return <div className="feed-loading">Connecting...</div>;
  }

  return (
    <div className="feed">
      <form className="post-form" onSubmit={handleSubmit}>
        <textarea
          value={content}
          onChange={(e) => setContent(e.target.value)}
          placeholder="What's happening?"
          rows={3}
        />
        <button type="submit" disabled={!content.trim()}>
          Post
        </button>
      </form>

      <div className="posts">
        {posts.map((post) => (
          <article key={post.id} className="post">
            <header className="post-header">
              <span className="post-author">@{post.author_id.slice(0, 8)}</span>
              {post.author_id !== peerId && (
                <button
                  type="button"
                  className="btn-add-friend"
                  onClick={() => requestFriend(post.author_id)}
                  title="Add friend"
                >
                  +
                </button>
              )}
              <time className="post-time">
                {new Date(post.created_at).toLocaleTimeString()}
              </time>
            </header>
            <p className="post-content">{post.content}</p>
            <footer className="post-footer">
              <button
                type="button"
                className="btn-like"
                onClick={() => likePost(post.id)}
              >
                ♥ {post.likes}
              </button>
              <span>↻ {post.reposts}</span>
            </footer>
          </article>
        ))}
      </div>
    </div>
  );
}
