#[doc = "Register `LINRD0` reader"]
pub type R = crate::R<Linrd0Spec>;
#[doc = "Register `LINRD0` writer"]
pub type W = crate::W<Linrd0Spec>;
#[doc = "Field `RD3` reader - 7:0\\]
8-bit Receive Buffer 3. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd3R = crate::FieldReader;
#[doc = "Field `RD3` writer - 7:0\\]
8-bit Receive Buffer 3. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD2` reader - 15:8\\]
8-bit Receive Buffer 2. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd2R = crate::FieldReader;
#[doc = "Field `RD2` writer - 15:8\\]
8-bit Receive Buffer 2. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD1` reader - 23:16\\]
8-bit Receive Buffer 1. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd1R = crate::FieldReader;
#[doc = "Field `RD1` writer - 23:16\\]
8-bit Receive Buffer 1. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
pub type Rd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RD0` reader - 31:24\\]
8-bit Receive Buffer 0 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received. A read of this byte clears the RXDY byte. Note: RD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
pub type Rd0R = crate::FieldReader;
#[doc = "Field `RD0` writer - 31:24\\]
8-bit Receive Buffer 0 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received. A read of this byte clears the RXDY byte. Note: RD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
pub type Rd0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Receive Buffer 3. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    pub fn rd3(&self) -> Rd3R {
        Rd3R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Receive Buffer 2. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    pub fn rd2(&self) -> Rd2R {
        Rd2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Receive Buffer 1. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    pub fn rd1(&self) -> Rd1R {
        Rd1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Receive Buffer 0 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received. A read of this byte clears the RXDY byte. Note: RD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
    #[inline(always)]
    pub fn rd0(&self) -> Rd0R {
        Rd0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
8-bit Receive Buffer 3. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    #[must_use]
    pub fn rd3(&mut self) -> Rd3W<Linrd0Spec> {
        Rd3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
8-bit Receive Buffer 2. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    #[must_use]
    pub fn rd2(&mut self) -> Rd2W<Linrd0Spec> {
        Rd2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
8-bit Receive Buffer 1. Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received."]
    #[inline(always)]
    #[must_use]
    pub fn rd1(&mut self) -> Rd1W<Linrd0Spec> {
        Rd1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
8-bit Receive Buffer 0 Each response data-byte that is received in the SCIRXSHFT register is transferred to the corresponding RDy register according to the number of bytes received. A read of this byte clears the RXDY byte. Note: RD&lt;x-1> is equivalent to Data byte &lt;x> of the LIN frame."]
    #[inline(always)]
    #[must_use]
    pub fn rd0(&mut self) -> Rd0W<Linrd0Spec> {
        Rd0W::new(self, 24)
    }
}
#[doc = "The LINRD0 register contains the lower 4 bytes of the received LIN frame data.\n\nYou can [`read`](crate::Reg::read) this register and get [`linrd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linrd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Linrd0Spec;
impl crate::RegisterSpec for Linrd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linrd0::R`](R) reader structure"]
impl crate::Readable for Linrd0Spec {}
#[doc = "`write(|w| ..)` method takes [`linrd0::W`](W) writer structure"]
impl crate::Writable for Linrd0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINRD0 to value 0"]
impl crate::Resettable for Linrd0Spec {
    const RESET_VALUE: u32 = 0;
}
