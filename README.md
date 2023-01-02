# Football-DAOs

DAOs for football clubs on Solana!

This project explores how community-centric organisations like sports clubs can offer new ways of engagement with their fans, and how access to high value investments like football clubs can be made accessible to everyday investors.

It aims to enable use-cases from offering actual ownership and control of a football club to fans to creating specialised fan-run DAO structures (like a fan club, or a stategy advisory organisation).

SlideShow link from Encode Club demo day: https://docs.google.com/presentation/d/1pPbqMPb4c--DtjuW7-pu2GPpQZzxL6WeuTsf9z3g7rE/edit?usp=sharing

## How it works

There are two main solana programs in this project. The first is a custom program that allows us to optimise the DAO creation flow specifically for football club use-cases. The second is a clone of the spl-governance program which allows us to efficiently create DAOs using a program that we own. The idea of cloning the spl-governance program instead of calling it directly is so that we have our own instance that we can put under governance, meaning that we can create a super DAO which manages the actual governance program.

Both programs are deployed to devnet with the following IDs:

Custom football-dao program: 3Sz5VQ2VnxZsgTsGrJqUSbpfM4H4efMm8QCFqLtq6WjN
Cloned spl-governance program: 5eVUjoF1uTzSA4eA4xEiL9Y1MrsboMcugWp4CC9Md6aZ

The front-end is pending.
