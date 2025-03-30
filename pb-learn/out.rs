mod module {
    use burn::{nn::Linear, prelude::*};
    pub struct BetaActor<B: Backend> {
        l1: Linear<B>,
        l2: Linear<B>,
        alpha_head: Linear<B>,
        beta_head: Linear<B>,
    }
    impl<B: Backend> burn::module::Module<B> for BetaActor<B> {
        type Record = BetaActorRecord<B>;
        fn load_record(self, record: Self::Record) -> Self {
            Self {
                l1: burn::module::Module::<B>::load_record(self.l1, record.l1),
                l2: burn::module::Module::<B>::load_record(self.l2, record.l2),
                alpha_head: burn::module::Module::<
                    B,
                >::load_record(self.alpha_head, record.alpha_head),
                beta_head: burn::module::Module::<
                    B,
                >::load_record(self.beta_head, record.beta_head),
            }
        }
        fn into_record(self) -> Self::Record {
            Self::Record {
                l1: burn::module::Module::<B>::into_record(self.l1),
                l2: burn::module::Module::<B>::into_record(self.l2),
                alpha_head: burn::module::Module::<B>::into_record(self.alpha_head),
                beta_head: burn::module::Module::<B>::into_record(self.beta_head),
            }
        }
        fn num_params(&self) -> usize {
            let mut num_params = 0;
            num_params += burn::module::Module::<B>::num_params(&self.l1);
            num_params += burn::module::Module::<B>::num_params(&self.l2);
            num_params += burn::module::Module::<B>::num_params(&self.alpha_head);
            num_params += burn::module::Module::<B>::num_params(&self.beta_head);
            num_params
        }
        fn visit<Visitor: burn::module::ModuleVisitor<B>>(&self, visitor: &mut Visitor) {
            burn::module::Module::visit(&self.l1, visitor);
            burn::module::Module::visit(&self.l2, visitor);
            burn::module::Module::visit(&self.alpha_head, visitor);
            burn::module::Module::visit(&self.beta_head, visitor);
        }
        fn map<Mapper: burn::module::ModuleMapper<B>>(
            self,
            mapper: &mut Mapper,
        ) -> Self {
            let l1 = burn::module::Module::<B>::map(self.l1, mapper);
            let l2 = burn::module::Module::<B>::map(self.l2, mapper);
            let alpha_head = burn::module::Module::<B>::map(self.alpha_head, mapper);
            let beta_head = burn::module::Module::<B>::map(self.beta_head, mapper);
            Self {
                l1,
                l2,
                alpha_head,
                beta_head,
            }
        }
        fn collect_devices(
            &self,
            devices: burn::module::Devices<B>,
        ) -> burn::module::Devices<B> {
            let devices = burn::module::Module::<B>::collect_devices(&self.l1, devices);
            let devices = burn::module::Module::<B>::collect_devices(&self.l2, devices);
            let devices = burn::module::Module::<
                B,
            >::collect_devices(&self.alpha_head, devices);
            let devices = burn::module::Module::<
                B,
            >::collect_devices(&self.beta_head, devices);
            devices
        }
        fn to_device(self, device: &B::Device) -> Self {
            let l1 = burn::module::Module::<B>::to_device(self.l1, device);
            let l2 = burn::module::Module::<B>::to_device(self.l2, device);
            let alpha_head = burn::module::Module::<
                B,
            >::to_device(self.alpha_head, device);
            let beta_head = burn::module::Module::<B>::to_device(self.beta_head, device);
            Self {
                l1,
                l2,
                alpha_head,
                beta_head,
            }
        }
        fn fork(self, device: &B::Device) -> Self {
            let l1 = burn::module::Module::<B>::fork(self.l1, device);
            let l2 = burn::module::Module::<B>::fork(self.l2, device);
            let alpha_head = burn::module::Module::<B>::fork(self.alpha_head, device);
            let beta_head = burn::module::Module::<B>::fork(self.beta_head, device);
            Self {
                l1,
                l2,
                alpha_head,
                beta_head,
            }
        }
    }
    impl<B: Backend> burn::module::AutodiffModule<B> for BetaActor<B>
    where
        B: burn::tensor::backend::AutodiffBackend,
        <B as burn::tensor::backend::AutodiffBackend>::InnerBackend: Backend,
    {
        type InnerModule = BetaActor<B::InnerBackend>;
        fn valid(&self) -> Self::InnerModule {
            let l1 = burn::module::AutodiffModule::<B>::valid(&self.l1);
            let l2 = burn::module::AutodiffModule::<B>::valid(&self.l2);
            let alpha_head = burn::module::AutodiffModule::<B>::valid(&self.alpha_head);
            let beta_head = burn::module::AutodiffModule::<B>::valid(&self.beta_head);
            Self::InnerModule {
                l1,
                l2,
                alpha_head,
                beta_head,
            }
        }
    }
    impl<B: Backend> core::fmt::Display for BetaActor<B> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let formatted = burn::module::ModuleDisplay::format(
                self,
                Default::default(),
            );
            f.write_fmt(format_args!("{0}", formatted))
        }
    }
    impl<B: Backend> burn::module::ModuleDisplayDefault for BetaActor<B> {
        fn content(
            &self,
            mut content: burn::module::Content,
        ) -> Option<burn::module::Content> {
            content
                .set_top_level_type(&"BetaActor")
                .add("l1", &self.l1)
                .add("l2", &self.l2)
                .add("alpha_head", &self.alpha_head)
                .add("beta_head", &self.beta_head)
                .optional()
        }
        fn num_params(&self) -> usize {
            burn::module::Module::num_params(self)
        }
    }
    impl<B: Backend> Clone for BetaActor<B> {
        fn clone(&self) -> Self {
            let l1 = self.l1.clone();
            let l2 = self.l2.clone();
            let alpha_head = self.alpha_head.clone();
            let beta_head = self.beta_head.clone();
            Self {
                l1,
                l2,
                alpha_head,
                beta_head,
            }
        }
    }
    /// The record type for the module.
    pub struct BetaActorRecord<B: Backend> {
        /// The module record associative type.
        pub l1: <Linear<B> as burn::module::Module<B>>::Record,
        /// The module record associative type.
        pub l2: <Linear<B> as burn::module::Module<B>>::Record,
        /// The module record associative type.
        pub alpha_head: <Linear<B> as burn::module::Module<B>>::Record,
        /// The module record associative type.
        pub beta_head: <Linear<B> as burn::module::Module<B>>::Record,
    }
    /// The record item type for the module.
    #[serde(crate = "burn::serde")]
    #[serde(
        bound = "< < Linear < B > as burn :: module :: Module < B > > :: Record as burn ::\nrecord :: Record < B >> :: Item < S > : burn :: serde :: Serialize + burn ::\nserde :: de :: DeserializeOwned, < < Linear < B > as burn :: module :: Module\n< B > > :: Record as burn :: record :: Record < B >> :: Item < S > : burn ::\nserde :: Serialize + burn :: serde :: de :: DeserializeOwned, < < Linear < B >\nas burn :: module :: Module < B > > :: Record as burn :: record :: Record < B\n>> :: Item < S > : burn :: serde :: Serialize + burn :: serde :: de ::\nDeserializeOwned, < < Linear < B > as burn :: module :: Module < B > > ::\nRecord as burn :: record :: Record < B >> :: Item < S > : burn :: serde ::\nSerialize + burn :: serde :: de :: DeserializeOwned,"
    )]
    pub struct BetaActorRecordItem<B: Backend, S: burn::record::PrecisionSettings> {
        /// Field to be serialized.
        pub l1: <<Linear<
            B,
        > as burn::module::Module<B>>::Record as burn::record::Record<B>>::Item<S>,
        /// Field to be serialized.
        pub l2: <<Linear<
            B,
        > as burn::module::Module<B>>::Record as burn::record::Record<B>>::Item<S>,
        /// Field to be serialized.
        pub alpha_head: <<Linear<
            B,
        > as burn::module::Module<B>>::Record as burn::record::Record<B>>::Item<S>,
        /// Field to be serialized.
        pub beta_head: <<Linear<
            B,
        > as burn::module::Module<B>>::Record as burn::record::Record<B>>::Item<S>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use burn::serde as _serde;
        #[automatically_derived]
        impl<B: Backend, S: burn::record::PrecisionSettings> burn::serde::Serialize
        for BetaActorRecordItem<B, S>
        where
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
        {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> burn::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: burn::serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "BetaActorRecordItem",
                    false as usize + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "l1",
                    &self.l1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "l2",
                    &self.l2,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "alpha_head",
                    &self.alpha_head,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "beta_head",
                    &self.beta_head,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use burn::serde as _serde;
        #[automatically_derived]
        impl<
            'de,
            B: Backend,
            S: burn::record::PrecisionSettings,
        > burn::serde::Deserialize<'de> for BetaActorRecordItem<B, S>
        where
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
            <<Linear<
                B,
            > as burn::module::Module<
                B,
            >>::Record as burn::record::Record<
                B,
            >>::Item<S>: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
        {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> burn::serde::__private::Result<Self, __D::Error>
            where
                __D: burn::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "l1" => _serde::__private::Ok(__Field::__field0),
                            "l2" => _serde::__private::Ok(__Field::__field1),
                            "alpha_head" => _serde::__private::Ok(__Field::__field2),
                            "beta_head" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"l1" => _serde::__private::Ok(__Field::__field0),
                            b"l2" => _serde::__private::Ok(__Field::__field1),
                            b"alpha_head" => _serde::__private::Ok(__Field::__field2),
                            b"beta_head" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de, B: Backend, S: burn::record::PrecisionSettings>
                where
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                {
                    marker: _serde::__private::PhantomData<BetaActorRecordItem<B, S>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<
                    'de,
                    B: Backend,
                    S: burn::record::PrecisionSettings,
                > _serde::de::Visitor<'de> for __Visitor<'de, B, S>
                where
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                    <<Linear<
                        B,
                    > as burn::module::Module<
                        B,
                    >>::Record as burn::record::Record<
                        B,
                    >>::Item<
                        S,
                    >: burn::serde::Serialize + burn::serde::de::DeserializeOwned,
                {
                    type Value = BetaActorRecordItem<B, S>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct BetaActorRecordItem",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct BetaActorRecordItem with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct BetaActorRecordItem with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct BetaActorRecordItem with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct BetaActorRecordItem with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(BetaActorRecordItem {
                            l1: __field0,
                            l2: __field1,
                            alpha_head: __field2,
                            beta_head: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        > = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        > = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        > = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            <<Linear<
                                B,
                            > as burn::module::Module<
                                B,
                            >>::Record as burn::record::Record<B>>::Item<S>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = _serde::de::MapAccess::next_key::<__Field>(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("l1"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            <<Linear<
                                                B,
                                            > as burn::module::Module<
                                                B,
                                            >>::Record as burn::record::Record<B>>::Item<S>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("l2"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            <<Linear<
                                                B,
                                            > as burn::module::Module<
                                                B,
                                            >>::Record as burn::record::Record<B>>::Item<S>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "alpha_head",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            <<Linear<
                                                B,
                                            > as burn::module::Module<
                                                B,
                                            >>::Record as burn::record::Record<B>>::Item<S>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "beta_head",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            <<Linear<
                                                B,
                                            > as burn::module::Module<
                                                B,
                                            >>::Record as burn::record::Record<B>>::Item<S>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("l1")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("l2")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("alpha_head")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("beta_head")?
                            }
                        };
                        _serde::__private::Ok(BetaActorRecordItem {
                            l1: __field0,
                            l2: __field1,
                            alpha_head: __field2,
                            beta_head: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "l1",
                    "l2",
                    "alpha_head",
                    "beta_head",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "BetaActorRecordItem",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<
                            BetaActorRecordItem<B, S>,
                        >,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl<B: Backend> burn::record::Record<B> for BetaActorRecord<B> {
        type Item<S: burn::record::PrecisionSettings> = BetaActorRecordItem<B, S>;
        fn into_item<S: burn::record::PrecisionSettings>(self) -> Self::Item<S> {
            BetaActorRecordItem {
                l1: burn::record::Record::<B>::into_item::<S>(self.l1),
                l2: burn::record::Record::<B>::into_item::<S>(self.l2),
                alpha_head: burn::record::Record::<B>::into_item::<S>(self.alpha_head),
                beta_head: burn::record::Record::<B>::into_item::<S>(self.beta_head),
            }
        }
        fn from_item<S: burn::record::PrecisionSettings>(
            item: Self::Item<S>,
            device: &B::Device,
        ) -> Self {
            Self {
                l1: burn::record::Record::<B>::from_item::<S>(item.l1, device),
                l2: burn::record::Record::<B>::from_item::<S>(item.l2, device),
                alpha_head: burn::record::Record::<
                    B,
                >::from_item::<S>(item.alpha_head, device),
                beta_head: burn::record::Record::<
                    B,
                >::from_item::<S>(item.beta_head, device),
            }
        }
    }
    impl<B: Backend> burn::module::ModuleDisplay for BetaActor<B> {}
    #[automatically_derived]
    impl<B: ::core::fmt::Debug + Backend> ::core::fmt::Debug for BetaActor<B> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "BetaActor",
                "l1",
                &self.l1,
                "l2",
                &self.l2,
                "alpha_head",
                &self.alpha_head,
                "beta_head",
                &&self.beta_head,
            )
        }
    }
}