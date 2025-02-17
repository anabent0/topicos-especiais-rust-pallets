#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::Vec;
    use scale_info::prelude::string::String;
    //use sp_std::str::FromStr;
    //use sp_runtime::traits::Zero;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + scale_info::TypeInfo {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type MaxTitleLength: Get<u32>;
        type MaxAuthorLength: Get<u32>;
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct Book<T: Config> {
        pub id: u32,
        pub title: BoundedVec<u8, T::MaxTitleLength>,
        pub pages: u32,
        pub publication_date: u32, // Armazenando como um valor numérico (ddmmyyyy)
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    pub struct User<T: Config> {
        pub id: u32,
        pub name: BoundedVec<u8, T::MaxTitleLength>,
        pub cpf: BoundedVec<u8, T::MaxTitleLength>,
        pub age: u32,
    }

    #[pallet::storage]
    #[pallet::getter(fn books)]
    pub type Books<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, u32, Book<T>>;

    #[pallet::storage]
    #[pallet::getter(fn users)]
    pub type Users<T: Config> = StorageMap<_, Blake2_128Concat, u32, User<T>>;

    #[pallet::storage]
    #[pallet::getter(fn book_counter)]
    pub type BookCounter<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn user_counter)]
    pub type UserCounter<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn borrowings)]
    pub type Borrowings<T: Config> = StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, ()>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BookCreated { who: T::AccountId, id: u32 },
        BookUpdated { who: T::AccountId, id: u32 },
        BookDeleted { who: T::AccountId, id: u32 },
        UserCreated { who: T::AccountId, id: u32 },
        UserUpdated { who: T::AccountId, id: u32 },
        UserDeleted { who: T::AccountId, id: u32 },
        BookBorrowed { who: T::AccountId, book_id: u32 },
        BookReturned { who: T::AccountId, book_id: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        BookNotFound,
        TitleTooLong,
        PagesCannotBeZero,
        InvalidPublicationDate,
        UserNotFound,
        CpfAlreadyExists,
        InvalidCpf,
        BookAlreadyBorrowed,
        BookNotBorrowedByUser,
    }

    // Função auxiliar para validar o CPF
    fn validate_cpf(cpf: &str) -> bool {
        // Lógica simples de validação do CPF (apenas valida o formato e comprimento)
        let cpf_digits = cpf.chars().filter(|c| c.is_digit(10)).collect::<String>();
        cpf_digits.len() == 11
    }

    // Função auxiliar para validar a data de publicação
    fn validate_publication_date<T: Config>(date: &str) -> Result<u32, Error<T>> {
        let parts: Vec<&str> = date.split('-').collect();
        if parts.len() == 3 {
            let day = parts[0].parse::<u32>().map_err(|_| Error::<T>::InvalidPublicationDate)?;
            let month = parts[1].parse::<u32>().map_err(|_| Error::<T>::InvalidPublicationDate)?;
            let year = parts[2].parse::<u32>().map_err(|_| Error::<T>::InvalidPublicationDate)?;

            // Validando o formato de data
            if day > 31 || month > 12 || year == 0 {
                return Err(Error::<T>::InvalidPublicationDate);
            }
            Ok(day * 1000000 + month * 10000 + year)
        } else {
            Err(Error::<T>::InvalidPublicationDate)
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn convert_to_timestamp(date_str: Vec<u8>) -> Result<u64, Error<T>> {
            let date_str = core::str::from_utf8(&date_str).map_err(|_| Error::<T>::InvalidPublicationDate)?;
            let parts: Vec<&str> = date_str.split('/').collect();
            if parts.len() != 3 {
                return Err(Error::<T>::InvalidPublicationDate);
            }
            let day: u32 = parts[0].parse().map_err(|_| Error::<T>::InvalidPublicationDate)?;
            let month: u32 = parts[1].parse().map_err(|_| Error::<T>::InvalidPublicationDate)?;
            let year: i32 = parts[2].parse().map_err(|_| Error::<T>::InvalidPublicationDate)?;
            if month < 1 || month > 12 || day < 1 || day > 31 {
                return Err(Error::<T>::InvalidPublicationDate);
            }
            let timestamp = Self::srt_timestamp(year, month, day)?;
            Ok(timestamp)
        }
    
        fn srt_timestamp(year: i32, month: u32, day: u32) -> Result<u64, Error<T>> {
            let mut days = 0;
            for y in 1970..year {
                days += if Self::is_bissexto(y) { 366 } else { 365 };
            }
            let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            for m in 0..(month as usize - 1) {
                days += days_in_month[m];
                if m == 1 && Self::is_bissexto(year) {
                    days += 1;
                }
            }
            days += day - 1;
            let timestamp = days as u64 * 86400;
            Ok(timestamp)
        }
    
        fn is_bissexto(year: i32) -> bool {
            (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0) 
        }
    }
    

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // CRUD de Livro
        #[pallet::weight(10_000)]
        #[pallet::call_index(0)]
        pub fn create_book(
            origin: OriginFor<T>,
            title: String,
            pages: u32,
            publication_date: String,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(pages > 0, Error::<T>::PagesCannotBeZero);

            let id = BookCounter::<T>::get(&who);
            let publication_date = validate_publication_date::<T>(&publication_date)?;

            let book = Book {
                id,
                title: BoundedVec::try_from(title.into_bytes()).map_err(|_| Error::<T>::TitleTooLong)?,
                pages,
                publication_date,
            };

            Books::<T>::insert(&who, id, book);
            BookCounter::<T>::insert(&who, id + 1);
            Self::deposit_event(Event::BookCreated { who, id });
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(1)]
        pub fn update_book(
            origin: OriginFor<T>,
            id: u32,
            title: String,
            pages: u32,
            publication_date: String,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Books::<T>::contains_key(&who, id), Error::<T>::BookNotFound);

            let publication_date = validate_publication_date::<T>(&publication_date)?;

            let book = Book {
                id,
                title: BoundedVec::try_from(title.into_bytes()).map_err(|_| Error::<T>::TitleTooLong)?,
                pages,
                publication_date,
            };

            Books::<T>::insert(&who, id, book);
            Self::deposit_event(Event::BookUpdated { who, id });
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(2)]
        pub fn delete_book(origin: OriginFor<T>, id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Books::<T>::contains_key(&who, id), Error::<T>::BookNotFound);

            Books::<T>::remove(&who, id);
            Self::deposit_event(Event::BookDeleted { who, id });
            Ok(())
        }

        // CRUD de Usuário
        #[pallet::weight(10_000)]
        #[pallet::call_index(3)]
        pub fn create_user(
            origin: OriginFor<T>,
            name: String,
            cpf: String,
            age: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(validate_cpf(&cpf), Error::<T>::InvalidCpf);

            let id = UserCounter::<T>::get();
            ensure!(!Users::<T>::contains_key(&id), Error::<T>::CpfAlreadyExists);

            let user = User {
                id,
                name: BoundedVec::try_from(name.into_bytes()).map_err(|_| Error::<T>::TitleTooLong)?,
                cpf: BoundedVec::try_from(cpf.into_bytes()).map_err(|_| Error::<T>::TitleTooLong)?,
                age,
            };

            Users::<T>::insert(id, user);
            UserCounter::<T>::put(id + 1);
            Self::deposit_event(Event::UserCreated { who, id });
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(4)]
        pub fn update_user(
            origin: OriginFor<T>,
            id: u32,
            name: String,
            cpf: String,
            age: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Users::<T>::contains_key(id), Error::<T>::UserNotFound);
            ensure!(validate_cpf(&cpf), Error::<T>::InvalidCpf);

            let user = User {
                id,
                name: BoundedVec::try_from(name.into_bytes()).map_err(|_| Error::<T>::TitleTooLong)?,
                cpf: BoundedVec::try_from(cpf.into_bytes()).map_err(|_| Error::<T>::TitleTooLong)?,
                age,
            };

            Users::<T>::insert(id, user);
            Self::deposit_event(Event::UserUpdated { who, id });
            Ok(())
        }

        #[pallet::weight(10_000)]
        #[pallet::call_index(5)]
        pub fn delete_user(origin: OriginFor<T>, id: u32) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(Users::<T>::contains_key(id), Error::<T>::UserNotFound);

            Users::<T>::remove(id);
            Self::deposit_event(Event::UserDeleted { who, id });
            Ok(())
        }
        // Função para emprestar um livro
        #[pallet::weight(10_000)]
        #[pallet::call_index(6)]
        pub fn borrow_book(
            origin: OriginFor<T>,
            book_id: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verifica se o livro existe no CRUD
            ensure!(Books::<T>::contains_key(&who, book_id), Error::<T>::BookNotFound);

            // Verifica se o livro já está emprestado
            ensure!(!Borrowings::<T>::contains_key(&book_id, &who), Error::<T>::BookAlreadyBorrowed);

            // Empresta o livro associando o id do livro ao usuário
            Borrowings::<T>::insert(&book_id, &who, ());

            // Emite evento de empréstimo
            Self::deposit_event(Event::BookBorrowed { who, book_id });

            Ok(())
        }

        
        // Função para devolver um livro
        #[pallet::weight(10_000)]
        #[pallet::call_index(7)]
        pub fn return_book(
            origin: OriginFor<T>,
            book_id: u32,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Verifica se o livro está emprestado ao usuário
            ensure!(Borrowings::<T>::contains_key(&book_id, &who), Error::<T>::BookNotBorrowedByUser);

            // Remove o livro do empréstimo
            Borrowings::<T>::remove(&book_id, &who);

            // Emite evento de devolução
            Self::deposit_event(Event::BookReturned { who, book_id });

            Ok(())
        }



    }
}
