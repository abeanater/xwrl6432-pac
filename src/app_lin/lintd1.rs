#[doc = "Register `LINTD1` reader"]
pub type R = crate::R<Lintd1Spec>;
#[doc = "Register `LINTD1` writer"]
pub type W = crate::W<Lintd1Spec>;
#[doc = "Field `TD7` reader - 7:0\\]
8-bit Transmit Buffer 7. Byte 7 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td7R = crate::FieldReader;
#[doc = "Field `TD7` writer - 7:0\\]
8-bit Transmit Buffer 7. Byte 7 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TD6` reader - 15:8\\]
8-bit Transmit Buffer 6. Byte 6 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td6R = crate::FieldReader;
#[doc = "Field `TD6` writer - 15:8\\]
8-bit Transmit Buffer 6. Byte 6 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TD5` reader - 23:16\\]
8-bit Transmit Buffer 5. Byte 5 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td5R = crate::FieldReader;
#[doc = "Field `TD5` writer - 23:16\\]
8-bit Transmit Buffer 5. Byte 5 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TD4` reader - 31:24\\]
8-bit Transmit Buffer 4. Byte4 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td4R = crate::FieldReader;
#[doc = "Field `TD4` writer - 31:24\\]
8-bit Transmit Buffer 4. Byte4 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
pub type Td4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Transmit Buffer 7. Byte 7 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    pub fn td7(&self) -> Td7R {
        Td7R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Transmit Buffer 6. Byte 6 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    pub fn td6(&self) -> Td6R {
        Td6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Transmit Buffer 5. Byte 5 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    pub fn td5(&self) -> Td5R {
        Td5R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Transmit Buffer 4. Byte4 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    pub fn td4(&self) -> Td4R {
        Td4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Transmit Buffer 7. Byte 7 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn td7(&mut self) -> Td7W<Lintd1Spec> {
        Td7W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Transmit Buffer 6. Byte 6 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn td6(&mut self) -> Td6W<Lintd1Spec> {
        Td6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Transmit Buffer 5. Byte 5 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn td5(&mut self) -> Td5W<Lintd1Spec> {
        Td5W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Transmit Buffer 4. Byte4 to be transmitted is written into this register and then copied to SCITXSHF for transmission."]
    #[inline(always)]
    #[must_use]
    pub fn td4(&mut self) -> Td4W<Lintd1Spec> {
        Td4W::new(self, 24)
    }
}
#[doc = "The LINTD1 register contains the upper 4 bytes of the data to be transmitted. NOTE: TD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame.\n\nYou can [`read`](crate::Reg::read) this register and get [`lintd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lintd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lintd1Spec;
impl crate::RegisterSpec for Lintd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lintd1::R`](R) reader structure"]
impl crate::Readable for Lintd1Spec {}
#[doc = "`write(|w| ..)` method takes [`lintd1::W`](W) writer structure"]
impl crate::Writable for Lintd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINTD1 to value 0"]
impl crate::Resettable for Lintd1Spec {
    const RESET_VALUE: u32 = 0;
}
