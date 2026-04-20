🌟 Stellar Notes DApp

A Secure Multi-User On-Chain Note Management System Built with Soroban

📌 Project Description

Stellar Notes DApp is a decentralized application built on the Stellar blockchain using the Soroban smart contract platform. This project enables users to create, manage, and delete personal notes directly on-chain in a secure and trustless environment.

Unlike traditional note-taking apps, this system ensures that each user's data is isolated, authenticated, and stored transparently on the blockchain. Every interaction requires wallet authorization, guaranteeing that only the owner of the notes can modify them.

The application demonstrates how decentralized storage and identity (via wallet addresses) can be used to build simple yet powerful Web3 applications.

🎯 Project Vision

This project aims to:

Demonstrate multi-user smart contract architecture
Showcase secure on-chain data management
Provide a learning reference for Soroban developers
Build a foundation for decentralized productivity tools

Rather than replacing modern apps, this project focuses on illustrating how real-world applications can be adapted to blockchain systems.

⚙️ Core Features
👤 1. User-Based Notes (Multi-User System)
Each user (wallet address) has their own notes
Data is isolated per user
No shared/global storage
🔐 2. Authentication & Security
Uses require_auth() for every write operation
Ensures only the note owner can create/delete notes
📝 3. Note Creation
Add notes with title and content
Automatically assigns unique incremental ID
Includes blockchain timestamp
📚 4. Retrieve Notes
Fetch all notes belonging to a specific user
Returns structured Vec<Note> data
Easy integration with frontend apps
🗑️ 5. Delete Notes
Remove notes using ID
Immediate state update on-chain
⏱️ 6. Timestamp Tracking
Each note stores creation time
Enables sorting and history tracking
🧱 Smart Contract Design
Data Structure
pub struct Note {
    id: u64,
    title: String,
    content: String,
    timestamp: u64,
}
Storage Model
Key	Value
Address	Vec<Note>
COUNTER	u64
Each user address maps to their own notes
Counter ensures unique IDs across all notes
🔧 Contract Functions
Function	Description
create_note	Create a new note
get_notes	Retrieve all notes for a user
delete_note	Delete a note by ID
⚠️ Design Considerations
❗ Notes are stored as Vec → may not scale well for large data
❗ No encryption → data is publicly visible
❗ No pagination → all notes fetched at once
🚀 Future Improvements
🔹 Short-Term
Input validation improvements
Pagination for notes
Better error handling
🔹 Medium-Term
🔍 Search & filtering
🏷️ Tags/categories
🔐 Client-side encryption
🔹 Long-Term
☁️ IPFS integration (off-chain storage)
🤝 Shared/collaborative notes
🧠 AI summarization
🪪 Decentralized identity (DID)
🛠️ Tech Stack
Rust
Soroban SDK
Stellar Blockchain

<img width="1919" height="897" alt="image" src="https://github.com/user-attachments/assets/ad76cf19-b704-4f66-a0a2-a16031b94c11" />


ID Contract: CD2FLTIZ5A5HFNEFAI6AWR5WIMAGBJ557D44PP7J52IHSUOIDM5RQR2Q
React + Tailwind (Frontend)
Freighter Wallet
