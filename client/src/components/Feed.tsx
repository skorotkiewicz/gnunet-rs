import {
	Globe,
	Heart,
	Lock,
	Repeat2,
	UserCheck,
	UserPlus,
	Users,
} from "lucide-react";
import { useState } from "react";
import { useSocial } from "../hooks";
import type { PostVisibility } from "../types";

export function Feed() {
	const {
		posts,
		createPost,
		connected,
		likePost,
		requestFriend,
		peerId,
		friends,
		showProfile,
	} = useSocial();
	const [content, setContent] = useState("");
	const [visibility, setVisibility] = useState<PostVisibility>("Public");

	const handleSubmit = (e: React.FormEvent) => {
		e.preventDefault();
		if (content.trim()) {
			createPost(content.trim(), visibility);
			setContent("");
			setVisibility("Public");
		}
	};

	if (!connected) {
		return <div className="feed-loading">Connecting...</div>;
	}

	const visibilityOptions: {
		value: PostVisibility;
		label: string;
		icon: React.ReactNode;
	}[] = [
		{ value: "Public", label: "Public", icon: <Globe size={14} /> },
		{ value: "FollowersOnly", label: "Followers", icon: <Users size={14} /> },
		{ value: "MutualsOnly", label: "Mutuals", icon: <UserCheck size={14} /> },
		{ value: "Private", label: "Private", icon: <Lock size={14} /> },
	];

	return (
		<div className="feed">
			<form className="post-form" onSubmit={handleSubmit}>
				<textarea
					value={content}
					onChange={(e) => setContent(e.target.value)}
					placeholder="What's happening?"
					rows={3}
				/>
				<div className="post-form-actions">
					<div className="visibility-selector">
						{visibilityOptions.map((opt) => (
							<button
								key={opt.value}
								type="button"
								className={`visibility-btn ${visibility === opt.value ? "active" : ""}`}
								onClick={() => setVisibility(opt.value)}
								title={opt.label}
							>
								{opt.icon}
							</button>
						))}
					</div>
					<button type="submit" disabled={!content.trim()}>
						Post
					</button>
				</div>
			</form>

			<div className="posts">
				{posts.map((post) => {
					const isLiked = peerId && post.likes.includes(peerId);
					const isFriend = friends.includes(post.author_id);
					return (
						<article key={post.id} className="post">
							<header className="post-header">
								<button
									type="button"
									className="post-author-btn"
									onClick={() => showProfile(post.author_id)}
								>
									@{post.author_id.slice(0, 8)}
								</button>
								{post.author_id !== peerId && !isFriend && (
									<button
										type="button"
										className="btn-add-friend"
										onClick={() => requestFriend(post.author_id)}
										title="Add friend"
									>
										<UserPlus size={14} />
									</button>
								)}
								<span className="post-visibility">
									{post.visibility === "Public" && <Globe size={12} />}
									{post.visibility === "FollowersOnly" && <Users size={12} />}
									{post.visibility === "MutualsOnly" && <UserCheck size={12} />}
									{post.visibility === "Private" && <Lock size={12} />}
								</span>
								<time className="post-time">
									{new Date(post.created_at).toLocaleTimeString()}
								</time>
							</header>
							<p className="post-content">{post.content}</p>
							<footer className="post-footer">
								<button
									type="button"
									className={`btn-like ${isLiked ? "liked" : ""}`}
									onClick={() => likePost(post.id)}
									title={isLiked ? "Unlike" : "Like"}
								>
									{isLiked ? (
										<Heart size={16} fill="currentColor" />
									) : (
										<Heart size={16} />
									)}
									<span>{post.likes.length}</span>
								</button>
								<button type="button" className="btn-repost" title="Repost">
									<Repeat2 size={16} />
									<span>{post.reposts}</span>
								</button>
							</footer>
						</article>
					);
				})}
			</div>
		</div>
	);
}
