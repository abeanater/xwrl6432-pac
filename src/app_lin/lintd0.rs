#[doc = "Register `LINTD0` reader"]
pub type R = crate::R<Lintd0Spec>;
#[doc = "Register `LINTD0` writer"]
pub type W = crate::W<Lintd0Spec>;
#[doc = "Field `TD3` reader - 7:0\\]
8-bit Transmit Buffer 3. Byte 3 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td3R = crate::FieldReader;
#[doc = "Field `TD3` writer - 7:0\\]
8-bit Transmit Buffer 3. Byte 3 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TD2` reader - 15:8\\]
8-bit Transmit Buffer 2. Byte 2 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td2R = crate::FieldReader;
#[doc = "Field `TD2` writer - 15:8\\]
8-bit Transmit Buffer 2. Byte 2 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TD1` reader - 23:16\\]
8-bit Transmit Buffer 3. Byte 1 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td1R = crate::FieldReader;
#[doc = "Field `TD1` writer - 23:16\\]
8-bit Transmit Buffer 3. Byte 1 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TD0` reader - 31:24\\]
8-bit Transmit Buffer 0. Byte 0 to be transmitted is written into this register and then copied to SCITXSHF for transmission. Once byte 0 is written in TDO buffer, transmission will be initiated."]
pub type Td0R = crate::FieldReader;
#[doc = "Field `TD0` writer - 31:24\\]
8-bit Transmit Buffer 0. Byte 0 to be transmitted is written into this register and then copied to SCITXSHF for transmission. Once byte 0 is written in TDO buffer, transmission will be initiated."]
pub type Td0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Transmit Buffer 3. Byte 3 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    pub fn td3(&self) -> Td3R {
        Td3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Transmit Buffer 2. Byte 2 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    pub fn td2(&self) -> Td2R {
        Td2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Transmit Buffer 3. Byte 1 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    pub fn td1(&self) -> Td1R {
        Td1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Transmit Buffer 0. Byte 0 to be transmitted is written into this register and then copied to SCITXSHF for transmission. Once byte 0 is written in TDO buffer, transmission will be initiated."]
    #[inline(always)]
    pub fn td0(&self) -> Td0R {
        Td0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Transmit Buffer 3. Byte 3 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn td3(&mut self) -> Td3W<Lintd0Spec> {
        Td3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Transmit Buffer 2. Byte 2 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn td2(&mut self) -> Td2W<Lintd0Spec> {
        Td2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Transmit Buffer 3. Byte 1 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn td1(&mut self) -> Td1W<Lintd0Spec> {
        Td1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Transmit Buffer 0. Byte 0 to be transmitted is written into this register and then copied to SCITXSHF for transmission. Once byte 0 is written in TDO buffer, transmission will be initiated."]
    #[inline(always)]
    #[must_use]
    pub fn td0(&mut self) -> Td0W<Lintd0Spec> {
        Td0W::new(self, 24)
    }
}
#[doc = "The LINTD0 register contains the lower 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`lintd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lintd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lintd0Spec;
impl crate::RegisterSpec for Lintd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lintd0::R`](R) reader structure"]
impl crate::Readable for Lintd0Spec {}
#[doc = "`write(|w| ..)` method takes [`lintd0::W`](W) writer structure"]
impl crate::Writable for Lintd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINTD0 to value 0"]
impl crate::Resettable for Lintd0Spec {
    const RESET_VALUE: u32 = 0;
}
