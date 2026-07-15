#[cfg(test)]
mod tests {
    use crate::bns::BnsRegistry;
    use crate::core::address::Address;

    #[test]
    fn test_bns_registration_and_resolution() {
        let mut reg = BnsRegistry::new();
        let alice = Address::from([1u8; 32]);
        let current_epoch = 10;
        
        // 1. Register a name
        reg.register("ayaz.bud".to_string(), alice, current_epoch, 100).unwrap();
        
        // 2. Resolve the name
        assert_eq!(reg.resolve("ayaz.bud", current_epoch + 1), Some(alice));
        
        // 3. Reject duplicate active registration
        let bob = Address::from([2u8; 32]);
        let err = reg.register("ayaz.bud".to_string(), bob, current_epoch + 5, 100).unwrap_err();
        assert!(matches!(err, crate::bns::BnsError::NameTaken));
    }

    #[test]
    fn test_bns_expiration() {
        let mut reg = BnsRegistry::new();
        let alice = Address::from([1u8; 32]);
        reg.register("expire.bud".to_string(), alice, 10, 10).unwrap();
        
        // Active at epoch 15
        assert_eq!(reg.resolve("expire.bud", 15), Some(alice));
        
        // Expired at epoch 25
        assert_eq!(reg.resolve("expire.bud", 25), None);
        
        // Can be re-registered after expiration
        let bob = Address::from([2u8; 32]);
        reg.register("expire.bud".to_string(), bob, 30, 100).unwrap();
        assert_eq!(reg.resolve("expire.bud", 35), Some(bob));
    }

    #[test]
    fn test_bns_full_impl_storage_binding() {
        let mut reg = BnsRegistry::new();
        let alice = Address::from([1u8; 32]);
        let storage_root = [0xAB; 32];
        let storage_domain_id = 5u32;

        reg.register_with_storage(
            "content.bud".to_string(),
            alice,
            10,
            100,
            storage_root,
            storage_domain_id,
        ).unwrap();

        let resolved = reg.resolve_full("content.bud", 15).unwrap();
        assert_eq!(resolved.owner, alice);
        assert_eq!(resolved.address, Some(alice));
        assert_eq!(resolved.storage_root, Some(storage_root));
        assert_eq!(resolved.storage_domain_id, Some(storage_domain_id));
        assert!(!resolved.is_expired);

        assert!(reg.resolve_full("content.bud", 200).is_none());
    }

    #[test]
    fn test_bns_set_storage_owner_only() {
        let mut reg = BnsRegistry::new();
        let alice = Address::from([1u8; 32]);
        let bob = Address::from([2u8; 32]);
        reg.register("update.bud".to_string(), alice, 10, 100).unwrap();

        let new_root = [0xCD; 32];
        let err = reg.set_storage("update.bud", bob, new_root, 5, 15).unwrap_err();
        assert!(matches!(err, crate::bns::BnsError::NotOwner));

        reg.set_storage("update.bud", alice, new_root, 5, 15).unwrap();
        let resolved = reg.resolve_full("update.bud", 20).unwrap();
        assert_eq!(resolved.storage_root, Some(new_root));
    }
}
