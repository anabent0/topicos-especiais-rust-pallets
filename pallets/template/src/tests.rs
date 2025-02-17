// Importações necessárias
use crate::{mock::*, Error};
use frame_support::{assert_ok,assert_err};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_book() {
        new_test_ext().execute_with(|| {
            let title = "Livro de Teste".to_string();
            let pages = 100;
            let publication_date = "01-01-2025".to_string();

            // Chame a função create_book do seu pallet
            assert_ok!(pallet_template::Pallet::<Test>::create_book(
                RuntimeOrigin::signed(1),
                title.clone(),
                pages,
                publication_date.clone(),
            ));

            // Recupere o livro criado e verifique os campos
            let book = pallet_template::Pallet::<Test>::books(1, 0).expect("Livro não encontrado");
			//let data_str = pallet_template::Pallet::<Test>::convert_to_timestamp(publication_date.into_bytes()).unwrap();
            assert_eq!(book.title, title.into_bytes());
            assert_eq!(book.pages, pages);
            
        });
    }

    #[test]
    fn test_update_book_success() {
        new_test_ext().execute_with(|| {
            let title = "Livro Atualizado".to_string();
            let pages = 120;
            let publication_date = "02-02-2025".to_string();

            // Criação do livro
            assert_ok!(pallet_template::Pallet::<Test>::create_book(
                RuntimeOrigin::signed(1),
                "Livro de Teste".to_string(),
                100,
                "01-01-2025".to_string(),
            ));

            // Atualiza o livro criado
            assert_ok!(pallet_template::Pallet::<Test>::update_book(
                RuntimeOrigin::signed(1),
                0,
                title.clone(),
                pages,
                publication_date.clone(),
            ));

            // Verifica se o livro foi atualizado
            let book = pallet_template::Pallet::<Test>::books(1, 0).expect("Livro não encontrado");
            assert_eq!(book.title, title.into_bytes());
            assert_eq!(book.pages, pages);
        });
    }


    #[test]
    fn test_create_user_success() {
        new_test_ext().execute_with(|| {
            let name = "Usuário Teste".to_string();
            let cpf = "12345678901".to_string();
            let age = 25;

            // Testa a criação de um usuário
            assert_ok!(pallet_template::Pallet::<Test>::create_user(
                RuntimeOrigin::signed(1),
                name.clone(),
                cpf.clone(),
                age,
            ));

            // Verifica se o usuário foi criado com os dados corretos
            let user = pallet_template::Pallet::<Test>::users(0).expect("Usuário não encontrado");
            assert_eq!(user.name, name.into_bytes());
            assert_eq!(user.cpf, cpf.into_bytes());
            assert_eq!(user.age, age);
        });
    }

	#[test]
	fn test_create_user_invalid_cpf() {
		new_test_ext().execute_with(|| {
			let name = "Usuário Inválido".to_string();
			let cpf = "12345".to_string();  // CPF inválido
			let age = 25;
	
			// Testa a criação de um usuário com CPF inválido
			let result = pallet_template::Pallet::<Test>::create_user(
				RuntimeOrigin::signed(1),
				name.clone(),
				cpf.clone(),
				age,
			);
	
			// Verifica se o erro retornado é o esperado
			assert_err!(result, Error::<Test>::InvalidCpf);
		});
	}
	

    #[test]
    fn test_update_user_success() {
        new_test_ext().execute_with(|| {
            let name = "Usuário Atualizado".to_string();
            let cpf = "12345678901".to_string();
            let age = 30;

            // Criação do usuário
            assert_ok!(pallet_template::Pallet::<Test>::create_user(
                RuntimeOrigin::signed(1),
                "Usuário Teste".to_string(),
                "12345678901".to_string(),
                25,
            ));

            // Atualiza o usuário criado
            assert_ok!(pallet_template::Pallet::<Test>::update_user(
                RuntimeOrigin::signed(1),
                0,
                name.clone(),
                cpf.clone(),
                age,
            ));

            // Verifica se o usuário foi atualizado
            let user = pallet_template::Pallet::<Test>::users(0).expect("Usuário não encontrado");
            assert_eq!(user.name, name.into_bytes());
            assert_eq!(user.cpf, cpf.into_bytes());
            assert_eq!(user.age, age);
        });
    }

    #[test]
	fn test_delete_user_success() {
		new_test_ext().execute_with(|| {
			let name = "Usuário Para Deletar".to_string();
			let cpf = "12345678901".to_string();
			let age = 25;

			// Criação do usuário
			assert_ok!(pallet_template::Pallet::<Test>::create_user(
				RuntimeOrigin::signed(1),
				name.clone(),
				cpf.clone(),
				age,
			));

			// Deleta o usuário criado
			assert_ok!(pallet_template::Pallet::<Test>::delete_user(
				RuntimeOrigin::signed(1),
				0
			));

			// Verifica se o usuário foi deletado
			let user = pallet_template::Pallet::<Test>::users(0);
			assert!(user.is_none());
		});
	}

	
}

