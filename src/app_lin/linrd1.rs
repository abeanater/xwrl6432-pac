#[doc = "Register `LINRD1` reader"]
pub type R = crate::R<Linrd1Spec>;
#[doc = "Register `LINRD1` writer"]
pub type W = crate::W<Linrd1Spec>;
#[doc = "Field `RD7` reader - 7:0\\]
8-bit Receive Buffer 7. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd7R = crate::FieldReader;
#[doc = "Field `RD7` writer - 7:0\\]
8-bit Receive Buffer 7. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD6` reader - 15:8\\]
8-bit Receive Buffer 6. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd6R = crate::FieldReader;
#[doc = "Field `RD6` writer - 15:8\\]
8-bit Receive Buffer 6. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD5` reader - 23:16\\]
8-bit Receive Buffer 5. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd5R = crate::FieldReader;
#[doc = "Field `RD5` writer - 23:16\\]
8-bit Receive Buffer 5. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD4` reader - 31:24\\]
8-bit Receive Buffer 4 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd4R = crate::FieldReader;
#[doc = "Field `RD4` writer - 31:24\\]
8-bit Receive Buffer 4 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Receive Buffer 7. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    pub fn rd7(&self) -> Rd7R {
        Rd7R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Receive Buffer 6. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    pub fn rd6(&self) -> Rd6R {
        Rd6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Receive Buffer 5. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    pub fn rd5(&self) -> Rd5R {
        Rd5R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Receive Buffer 4 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    pub fn rd4(&self) -> Rd4R {
        Rd4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Receive Buffer 7. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    #[must_use]
    pub fn rd7(&mut self) -> Rd7W<Linrd1Spec> {
        Rd7W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Receive Buffer 6. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    #[must_use]
    pub fn rd6(&mut self) -> Rd6W<Linrd1Spec> {
        Rd6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Receive Buffer 5. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    #[must_use]
    pub fn rd5(&mut self) -> Rd5W<Linrd1Spec> {
        Rd5W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Receive Buffer 4 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    #[must_use]
    pub fn rd4(&mut self) -> Rd4W<Linrd1Spec> {
        Rd4W::new(self, 24)
    }
}
#[doc = "The LINRD1 regsiter contains the upper 4 bytes of the received LIN frame data.\n\nYou can [`read`](crate::Reg::read) this register and get [`linrd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linrd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Linrd1Spec;
impl crate::RegisterSpec for Linrd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linrd1::R`](R) reader structure"]
impl crate::Readable for Linrd1Spec {}
#[doc = "`write(|w| ..)` method takes [`linrd1::W`](W) writer structure"]
impl crate::Writable for Linrd1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINRD1 to value 0"]
impl crate::Resettable for Linrd1Spec {
    const RESET_VALUE: u32 = 0;
}
