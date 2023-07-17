use crate::{mock::*, tipos::ProjectName, Error, Projects};
use codec::Encode;
use frame_support::{assert_noop, assert_ok};

#[test]
fn crear_proyecto_funciona() {
	new_test_ext().execute_with(|| {
		let nombre_corto = "A".encode();
		let nombre_largo = "Supercalifragilisticoexpialidoso".encode();
		let nombre_proyecto = "Proyecto #1".encode();
		let nombre_acotado: ProjectName<Test> = nombre_proyecto.clone().try_into().unwrap();
		assert_eq!(Projects::<Test>::contains_key(nombre_acotado.clone()), false);
		assert_noop!(
			Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_corto),
			Error::<Test>::NameTooShort
		);
		assert_noop!(
			Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_largo),
			Error::<Test>::NameTooLong
		);
		assert_ok!(Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_proyecto));
		assert_eq!(Projects::<Test>::contains_key(nombre_acotado.clone()), true);
		assert_eq!(Projects::<Test>::get(nombre_acotado), 0);
	});
}

#[test]
fn apoyar_proyecto_funciona() {
	new_test_ext().execute_with(|| {
		let nombre_proyecto: Vec<u8> = "Mi proyecto".encode();
		let nombre_acotado: ProjectName<Test> = nombre_proyecto.clone().try_into().unwrap();
		assert_ok!(Crowdfund::crear_proyecto(RuntimeOrigin::signed(1), nombre_proyecto.clone()));
		assert_ok!(Crowdfund::apoyar_proyecto(RuntimeOrigin::signed(2), nombre_proyecto, 500));
		assert_eq!(Projects::<Test>::get(nombre_acotado), 500);
	});
}
